/// Shared command-line driver for ecosystem scanner binaries.
///
/// `confrisk-npm` and `confrisk-gradle` (and any future `confrisk-<eco>`) differ
/// only in *which* [`Scanner`] they construct and a few display strings. All the
/// argument parsing, config resolution, output rendering and CI exit-code logic
/// lives here so each binary stays a thin entry point:
///
/// ```ignore
/// fn main() {
///     confrisk::cli::run(
///         confrisk::cli::ToolInfo::NPM,
///         |config, path| confrisk::npm::NpmScanner::new(config, path),
///     );
/// }
/// ```
use crate::config::{Config, CONFIG_DIR_ENV};
use crate::model::{score_all, AssetCriticality, ScoredFinding};
use crate::scanner::Scanner;
use std::process;

/// Static, per-binary display metadata. Construct via the associated constants
/// ([`ToolInfo::NPM`], [`ToolInfo::GRADLE`]) rather than by hand.
#[derive(Clone, Copy)]
pub struct ToolInfo {
    /// Executable name, e.g. `"confrisk-npm"`.
    pub bin: &'static str,
    /// Short version tag shown in usage/banner, e.g. `"v0.2"`.
    pub version: &'static str,
    /// One-line description of what the scanner does.
    pub tagline: &'static str,
    /// Banner title shown above results, e.g. `"NPM Security Scan"`.
    pub title: &'static str,
    /// Human label for the scanned project, e.g. `"npm project"`.
    pub project_kind: &'static str,
}

impl ToolInfo {
    pub const NPM: ToolInfo = ToolInfo {
        bin: "confrisk-npm",
        version: "v0.2",
        tagline: "NPM dependency security scanner",
        title: "NPM Security Scan",
        project_kind: "npm project",
    };

    pub const GRADLE: ToolInfo = ToolInfo {
        bin: "confrisk-gradle",
        version: "v0.2",
        tagline: "Gradle dependency security scanner",
        title: "Gradle Security Scan",
        project_kind: "Gradle project",
    };
}

/// Output format for scan results.
#[derive(Clone, Copy, PartialEq)]
pub enum OutputFormat {
    Text,
    Json,
    /// Self-contained HTML report written to `--out` (same template as the system scanner).
    Html,
}

/// Severity threshold at which a scan fails the build (`--fail-on`).
#[derive(Clone, Copy)]
pub enum FailLevel {
    Critical,
    High,
    Medium,
    Low,
}

impl FailLevel {
    /// The minimum scored risk that trips this threshold.
    fn threshold_risk(self) -> f64 {
        match self {
            FailLevel::Critical => 9.0,
            FailLevel::High => 6.0,
            FailLevel::Medium => 3.5,
            FailLevel::Low => 1.5,
        }
    }

    fn parse(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "critical" => Some(FailLevel::Critical),
            "high" => Some(FailLevel::High),
            "medium" => Some(FailLevel::Medium),
            "low" => Some(FailLevel::Low),
            _ => None,
        }
    }
}

/// Parsed, validated command-line arguments shared by every scanner binary.
pub struct ScanArgs {
    pub project_path: String,
    pub asset: AssetCriticality,
    pub format: OutputFormat,
    /// `Some` only when `--config` was passed; otherwise the config directory is
    /// resolved from `$CONFRISK_CONFIG_DIR` and standard locations.
    pub config_path: Option<String>,
    pub fail_on: FailLevel,
    pub exit_on_vuln: bool,
    /// Destination file for the HTML report (used only when `format` is `Html`).
    pub out_path: String,
}

/// Outcome of parsing arguments.
enum ParseOutcome {
    /// Proceed with a scan using these arguments.
    Run(ScanArgs),
    /// `--help` was requested; print usage and exit successfully.
    Help,
}

/// The full lifecycle of a scanner binary: parse args, resolve & load config,
/// build the scanner, scan, score, render, and exit with the right code.
///
/// `build` constructs the concrete [`Scanner`] from the loaded config and the
/// resolved project path. Never returns — always exits the process.
pub fn run<S: Scanner>(info: ToolInfo, build: impl FnOnce(Config, String) -> S) -> ! {
    let args: Vec<String> = std::env::args().collect();

    let parsed = match parse_args(info, &args) {
        Ok(ParseOutcome::Run(a)) => a,
        Ok(ParseOutcome::Help) => {
            print_usage(info);
            process::exit(0);
        }
        Err(msg) => {
            eprintln!("Error: {}", msg);
            print_usage(info);
            process::exit(1);
        }
    };

    // Resolve the config directory: --config flag, then $CONFRISK_CONFIG_DIR,
    // then standard Linux locations. See Config::resolve_dir.
    let config = match Config::load_resolved(parsed.config_path.as_deref()) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error loading config: {}", e);
            eprintln!(
                "Set the {} environment variable to your config directory, or pass --config <PATH>.",
                CONFIG_DIR_ENV
            );
            process::exit(1);
        }
    };

    let scanner = build(config, parsed.project_path.clone());
    let scored = score_all(scanner.scan(), parsed.asset);

    match parsed.format {
        OutputFormat::Json => render_json(&scored),
        OutputFormat::Text => render_text(info, &scored, &parsed.project_path),
        OutputFormat::Html => {
            let html = crate::report::render_project(
                &scored,
                parsed.asset,
                &parsed.project_path,
                &get_date(),
            );
            match std::fs::write(&parsed.out_path, html) {
                Ok(_) => println!("Raport HTML zapisany: {}", parsed.out_path),
                Err(e) => {
                    eprintln!("Error: failed to write report to '{}': {}", parsed.out_path, e);
                    process::exit(1);
                }
            }
        }
    }

    if parsed.exit_on_vuln && exit_code(&scored, parsed.fail_on) > 0 {
        eprintln!("\n❌ Security issues found! Failing build.");
        process::exit(exit_code(&scored, parsed.fail_on));
    }

    process::exit(0);
}

fn parse_args(info: ToolInfo, args: &[String]) -> Result<ParseOutcome, String> {
    let mut project_path = ".".to_string();
    let mut asset_str = "production".to_string();
    let mut format = OutputFormat::Text;
    let mut config_path: Option<String> = None;
    let mut fail_on = FailLevel::High;
    let mut exit_on_vuln = false;
    let mut out_path = "report.html".to_string();

    // Pull the value following a flag, erroring with a consistent message.
    let value_at = |i: usize, flag: &str| -> Result<String, String> {
        args.get(i + 1)
            .cloned()
            .ok_or_else(|| format!("{} requires a value", flag))
    };

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--path" | "-p" => {
                project_path = value_at(i, "--path")?;
                i += 2;
            }
            "--asset" | "-a" => {
                asset_str = value_at(i, "--asset")?;
                i += 2;
            }
            "--format" | "-f" => {
                let v = value_at(i, "--format")?;
                format = match v.as_str() {
                    "text" => OutputFormat::Text,
                    "json" => OutputFormat::Json,
                    "html" => OutputFormat::Html,
                    _ => return Err(format!("invalid format '{}'. Valid: text, json, html", v)),
                };
                i += 2;
            }
            "--out" | "-o" => {
                out_path = value_at(i, "--out")?;
                i += 2;
            }
            "--config" | "-c" => {
                config_path = Some(value_at(i, "--config")?);
                i += 2;
            }
            "--fail-on" => {
                let v = value_at(i, "--fail-on")?;
                fail_on = FailLevel::parse(&v)
                    .ok_or_else(|| format!("invalid --fail-on '{}'. Valid: critical, high, medium, low", v))?;
                exit_on_vuln = true;
                i += 2;
            }
            "--exit-code" => {
                exit_on_vuln = true;
                i += 1;
            }
            "--help" | "-h" => return Ok(ParseOutcome::Help),
            other => return Err(format!("unknown argument '{}'", other)),
        }
    }

    let asset = AssetCriticality::from_str(&asset_str).ok_or_else(|| {
        format!(
            "invalid asset profile '{}'. Valid values: dev, internal, production, crown-jewel",
            asset_str
        )
    })?;
    // `info` is unused here today but kept in the signature so per-tool argument
    // validation can be added without touching call sites.
    let _ = info;

    Ok(ParseOutcome::Run(ScanArgs {
        project_path,
        asset,
        format,
        config_path,
        fail_on,
        exit_on_vuln,
        out_path,
    }))
}

/// Current date/time as a display string, via the system `date` command.
fn get_date() -> String {
    if let Ok(out) = process::Command::new("date").arg("+%Y-%m-%d %H:%M:%S").output() {
        if out.status.success() {
            if let Ok(s) = String::from_utf8(out.stdout) {
                return s.trim().to_string();
            }
        }
    }
    "—".to_string()
}

fn print_usage(info: ToolInfo) {
    println!("{} {} — {}", info.bin, info.version, info.tagline);
    println!();
    println!("USAGE:");
    println!("    {} [OPTIONS]", info.bin);
    println!();
    println!("OPTIONS:");
    println!("    -p, --path <PATH>        Path to {} (default: .)", info.project_kind);
    println!("    -a, --asset <PROFILE>    Asset criticality: dev, internal, production, crown-jewel");
    println!("                             (default: production)");
    println!("    -f, --format <FORMAT>    Output format: text, json, html (default: text)");
    println!("    -o, --out <FILE>         HTML report destination (default: report.html)");
    println!("    -c, --config <PATH>      Config directory path. Overrides ${}.", CONFIG_DIR_ENV);
    println!("    --fail-on <LEVEL>        Fail build on: critical, high, medium, low");
    println!("                             (default: high)");
    println!("    --exit-code              Exit with non-zero code if vulnerabilities found");
    println!("    -h, --help               Show this help message");
    println!();
    println!("ENVIRONMENT:");
    println!("    {}      Directory holding the JSON config (rules, checks,", CONFIG_DIR_ENV);
    println!("                             plugins, scoring). Preferred way to point all");
    println!("                             scanners at one config. Falls back to standard");
    println!("                             paths (/etc/confrisk, ~/.config/confrisk, ./config).");
    println!();
    println!("EXAMPLES:");
    println!("    {}", info.bin);
    println!("    {} --path /path/to/project", info.bin);
    println!("    {} --fail-on high --exit-code", info.bin);
    println!("    {} --format json > results.json", info.bin);
    println!("    {} --format html --out raport.html", info.bin);
    println!("    {} --asset dev", info.bin);
}

/// Tally of unpassed findings by risk band.
struct Summary {
    critical: u32,
    high: u32,
    medium: u32,
    low: u32,
    passed: u32,
}

impl Summary {
    fn of(findings: &[ScoredFinding]) -> Self {
        let mut s = Summary { critical: 0, high: 0, medium: 0, low: 0, passed: 0 };
        for sf in findings {
            if sf.finding.passed {
                s.passed += 1;
                continue;
            }
            match sf.risk_band() {
                "critical" => s.critical += 1,
                "high" => s.high += 1,
                "medium" => s.medium += 1,
                "low" => s.low += 1,
                _ => {}
            }
        }
        s
    }

    fn has_issues(&self) -> bool {
        self.critical > 0 || self.high > 0 || self.medium > 0 || self.low > 0
    }
}

fn render_text(info: ToolInfo, findings: &[ScoredFinding], project_path: &str) {
    let banner = format!("  {} — {}", info.bin, info.title);
    println!("┌────────────────────────────────────────────────────────────┐");
    println!("│{:<60}│", banner);
    println!("└────────────────────────────────────────────────────────────┘");
    println!();
    println!("Project: {}", project_path);
    println!("Checks: {}", findings.len());
    println!();

    let summary = Summary::of(findings);

    println!("┌─ Summary ─────────────────────────────────────────────────┐");
    println!("│ Critical: {:3}                                              │", summary.critical);
    println!("│ High:     {:3}                                              │", summary.high);
    println!("│ Medium:   {:3}                                              │", summary.medium);
    println!("│ Low:      {:3}                                              │", summary.low);
    println!("│ Passed:   {:3}                                              │", summary.passed);
    println!("└───────────────────────────────────────────────────────────┘");
    println!();

    if !summary.has_issues() {
        println!("✅ No security issues found!");
        return;
    }

    println!("┌─ Issues Found (sorted by priority) ───────────────────────┐");
    println!();
    for (idx, sf) in findings.iter().enumerate() {
        if sf.finding.passed {
            continue;
        }
        let icon = match sf.risk_band() {
            "critical" => "[CRITICAL]",
            "high" => "[HIGH]",
            "medium" => "[MEDIUM]",
            "low" => "[LOW]",
            _ => "[NONE]",
        };
        println!(
            "{} [{}] {} (priority: {:.1})",
            icon, sf.finding.id, sf.finding.title, sf.priority
        );
        println!("   {}", sf.finding.description);
        println!("   Evidence: {}", sf.finding.evidence);
        println!("   Fix: {}", sf.finding.remediation);
        if idx < findings.len() - 1 && !findings[idx + 1].finding.passed {
            println!();
        }
    }
    println!();
    println!("└───────────────────────────────────────────────────────────┘");
}

fn render_json(findings: &[ScoredFinding]) {
    let output: Vec<serde_json::Value> = findings
        .iter()
        .map(|sf| {
            serde_json::json!({
                "id": sf.finding.id,
                "title": sf.finding.title,
                "description": sf.finding.description,
                "severity": sf.finding.severity.label(),
                "risk": sf.risk,
                "priority": sf.priority,
                "risk_band": sf.risk_band(),
                "evidence": sf.finding.evidence,
                "remediation": sf.finding.remediation,
                "passed": sf.finding.passed,
                "confidence": sf.finding.confidence,
                "effort": sf.finding.effort,
            })
        })
        .collect();

    println!("{}", serde_json::to_string_pretty(&output).unwrap());
}

/// Returns 1 if any unpassed finding meets the `fail_on` risk threshold, else 0.
fn exit_code(findings: &[ScoredFinding], fail_on: FailLevel) -> i32 {
    let threshold = fail_on.threshold_risk();
    if findings.iter().any(|sf| !sf.finding.passed && sf.risk >= threshold) {
        1
    } else {
        0
    }
}
