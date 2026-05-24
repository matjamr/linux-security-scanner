/// confrisk-npm — NPM dependency security scanner
///
/// Scans npm projects for vulnerable dependencies, blocked packages,
/// and security issues. Can be integrated into CI/CD and git hooks.

use std::env;
use std::process;

// Import from the main crate
use confrisk::config::Config;
use confrisk::model::{score_all, AssetCriticality, ScoredFinding};
use confrisk::npm::NpmScanner;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut project_path = ".".to_string();
    let mut asset_str = "production";
    let mut output_format = "text"; // text, json, html
    let mut exit_on_vuln = false;
    let mut config_path = "config";
    let mut fail_on = "high"; // critical, high, medium, low

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--path" | "-p" => {
                if i + 1 < args.len() {
                    project_path = args[i + 1].clone();
                    i += 2;
                } else {
                    eprintln!("Error: --path requires a value");
                    print_usage();
                    process::exit(1);
                }
            }
            "--asset" | "-a" => {
                if i + 1 < args.len() {
                    asset_str = &args[i + 1];
                    i += 2;
                } else {
                    eprintln!("Error: --asset requires a value");
                    print_usage();
                    process::exit(1);
                }
            }
            "--format" | "-f" => {
                if i + 1 < args.len() {
                    output_format = &args[i + 1];
                    i += 2;
                } else {
                    eprintln!("Error: --format requires a value");
                    print_usage();
                    process::exit(1);
                }
            }
            "--config" | "-c" => {
                if i + 1 < args.len() {
                    config_path = &args[i + 1];
                    i += 2;
                } else {
                    eprintln!("Error: --config requires a value");
                    print_usage();
                    process::exit(1);
                }
            }
            "--fail-on" => {
                if i + 1 < args.len() {
                    fail_on = &args[i + 1];
                    exit_on_vuln = true;
                    i += 2;
                } else {
                    eprintln!("Error: --fail-on requires a value");
                    print_usage();
                    process::exit(1);
                }
            }
            "--exit-code" => {
                exit_on_vuln = true;
                i += 1;
            }
            "--help" | "-h" => {
                print_usage();
                process::exit(0);
            }
            _ => {
                eprintln!("Error: unknown argument '{}'", args[i]);
                print_usage();
                process::exit(1);
            }
        }
    }

    // Load configuration
    let config = match Config::load(config_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error loading config: {}", e);
            eprintln!("Using current directory, config path: {}", config_path);
            eprintln!("Make sure config directory exists relative to current directory");
            process::exit(1);
        }
    };

    // Map asset string to AssetCriticality
    let asset_ctx = match AssetCriticality::from_str(asset_str) {
        Some(ctx) => ctx,
        None => {
            eprintln!(
                "Error: invalid asset profile '{}'. Valid values: dev, internal, production, crown-jewel",
                asset_str
            );
            process::exit(1);
        }
    };

    // Create scanner
    let scanner = NpmScanner::new(config, project_path.clone());

    // Run scan
    let findings = scanner.scan();

    // Score findings
    let scored_findings = score_all(findings, asset_ctx);

    // Output results
    match output_format {
        "json" => print_json(&scored_findings),
        "html" => {
            eprintln!("HTML output not yet implemented for npm scanner");
            print_text(&scored_findings, &project_path);
        }
        _ => print_text(&scored_findings, &project_path),
    }

    // Exit with code if requested
    if exit_on_vuln {
        let exit_code = get_exit_code(&scored_findings, fail_on);
        if exit_code > 0 {
            eprintln!("\n❌ Security issues found! Failing build.");
            process::exit(exit_code);
        }
    }
}

fn print_usage() {
    println!("confrisk-npm v0.2 — NPM dependency security scanner");
    println!();
    println!("USAGE:");
    println!("    confrisk-npm [OPTIONS]");
    println!();
    println!("OPTIONS:");
    println!("    -p, --path <PATH>        Path to npm project (default: .)");
    println!("    -a, --asset <PROFILE>    Asset criticality: dev, internal, production, crown-jewel");
    println!("                             (default: production)");
    println!("    -f, --format <FORMAT>    Output format: text, json (default: text)");
    println!("    -c, --config <PATH>      Config directory path (default: config)");
    println!("    --fail-on <LEVEL>        Fail build on: critical, high, medium, low");
    println!("                             (default: high)");
    println!("    --exit-code              Exit with non-zero code if vulnerabilities found");
    println!("    -h, --help               Show this help message");
    println!();
    println!("EXAMPLES:");
    println!("    # Scan current directory");
    println!("    confrisk-npm");
    println!();
    println!("    # Scan specific project");
    println!("    confrisk-npm --path /path/to/project");
    println!();
    println!("    # Fail CI on high or critical vulnerabilities");
    println!("    confrisk-npm --fail-on high --exit-code");
    println!();
    println!("    # JSON output for parsing");
    println!("    confrisk-npm --format json > results.json");
    println!();
    println!("    # Dev environment (lower risk weights)");
    println!("    confrisk-npm --asset dev");
}

fn print_text(findings: &[ScoredFinding], project_path: &str) {
    println!("┌────────────────────────────────────────────────────────────┐");
    println!("│  confrisk-npm — NPM Security Scan                          │");
    println!("└────────────────────────────────────────────────────────────┘");
    println!();
    println!("Project: {}", project_path);
    println!("Checks: {}", findings.len());
    println!();

    let mut critical = 0;
    let mut high = 0;
    let mut medium = 0;
    let mut low = 0;
    let mut passed = 0;

    for sf in findings {
        if sf.finding.passed {
            passed += 1;
        } else {
            match sf.risk_band() {
                "critical" => critical += 1,
                "high" => high += 1,
                "medium" => medium += 1,
                "low" => low += 1,
                _ => {}
            }
        }
    }

    println!("┌─ Summary ─────────────────────────────────────────────────┐");
    println!("│ Critical: {:3}                                              │", critical);
    println!("│ High:     {:3}                                              │", high);
    println!("│ Medium:   {:3}                                              │", medium);
    println!("│ Low:      {:3}                                              │", low);
    println!("│ Passed:   {:3}                                              │", passed);
    println!("└───────────────────────────────────────────────────────────┘");
    println!();

    if critical > 0 || high > 0 || medium > 0 || low > 0 {
        println!("┌─ Issues Found (sorted by priority) ───────────────────────┐");
        println!();

        for (idx, sf) in findings.iter().enumerate() {
            if sf.finding.passed {
                continue;
            }

            let icon = match sf.risk_band() {
                "critical" => "🔴",
                "high" => "🟠",
                "medium" => "🟡",
                "low" => "🔵",
                _ => "⚪",
            };

            println!(
                "{} [{}] {} (priority: {:.1})",
                icon,
                sf.finding.id,
                sf.finding.title,
                sf.priority
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
    } else {
        println!("✅ No security issues found!");
    }
}

fn print_json(findings: &[ScoredFinding]) {
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

fn get_exit_code(findings: &[ScoredFinding], fail_on: &str) -> i32 {
    let threshold_risk = match fail_on {
        "critical" => 9.0,
        "high" => 6.0,
        "medium" => 3.5,
        "low" => 1.5,
        _ => 6.0,
    };

    for sf in findings {
        if !sf.finding.passed && sf.risk >= threshold_risk {
            return 1;
        }
    }

    0
}
