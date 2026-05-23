/// confrisk — Linux security configuration scanner with risk-based prioritization

mod checks;
mod model;
mod report;

use model::AssetCriticality;
use std::env;
use std::fs;
use std::process::Command;

fn main() {
    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();

    let mut asset_str = "production";
    let mut output_path = "report.html";

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--asset" => {
                if i + 1 < args.len() {
                    asset_str = &args[i + 1];
                    i += 2;
                } else {
                    eprintln!("Error: --asset requires a value");
                    print_usage();
                    std::process::exit(1);
                }
            }
            "--out" => {
                if i + 1 < args.len() {
                    output_path = &args[i + 1];
                    i += 2;
                } else {
                    eprintln!("Error: --out requires a value");
                    print_usage();
                    std::process::exit(1);
                }
            }
            "--help" | "-h" => {
                print_usage();
                std::process::exit(0);
            }
            _ => {
                eprintln!("Error: unknown argument '{}'", args[i]);
                print_usage();
                std::process::exit(1);
            }
        }
    }

    // Map asset string to AssetCriticality
    let asset_ctx = match AssetCriticality::from_str(asset_str) {
        Some(ctx) => ctx,
        None => {
            eprintln!(
                "Error: invalid asset profile '{}'. Valid values: dev, internal, production, crown-jewel",
                asset_str
            );
            std::process::exit(1);
        }
    };

    // Get hostname
    let hostname = get_hostname();

    // Get scan date
    let scan_date = get_date();

    // Run all checks
    let findings = checks::run_all();

    // Score and prioritize
    let scored_findings = model::score_all(findings, asset_ctx);

    // Generate report
    let html = report::render(&scored_findings, asset_ctx, &hostname, &scan_date);

    // Write to file
    match fs::write(output_path, html) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error: failed to write report to '{}': {}", output_path, e);
            std::process::exit(1);
        }
    }

    // Print summary to stdout
    print_summary(&scored_findings, &hostname, asset_ctx, output_path);
}

fn print_usage() {
    println!("confrisk v0.1 — Linux security configuration scanner");
    println!();
    println!("USAGE:");
    println!("    confrisk [OPTIONS]");
    println!();
    println!("OPTIONS:");
    println!("    --asset <PROFILE>    Asset criticality profile");
    println!("                         Values: dev, internal, production, crown-jewel");
    println!("                         Default: production");
    println!();
    println!("    --out <FILE>         Output HTML report path");
    println!("                         Default: report.html");
    println!();
    println!("    --help, -h           Show this help message");
    println!();
    println!("EXAMPLES:");
    println!("    confrisk --asset dev --out dev-report.html");
    println!("    confrisk --asset crown-jewel --out /tmp/security-scan.html");
}

fn get_hostname() -> String {
    // Try to get hostname via command
    if let Ok(output) = Command::new("hostname").output() {
        if output.status.success() {
            if let Ok(hostname) = String::from_utf8(output.stdout) {
                return hostname.trim().to_string();
            }
        }
    }

    // Fallback: try /etc/hostname
    if let Ok(content) = fs::read_to_string("/etc/hostname") {
        return content.trim().to_string();
    }

    // Final fallback
    "unknown-host".to_string()
}

fn get_date() -> String {
    // Try to get date via command
    if let Ok(output) = Command::new("date").arg("+%Y-%m-%d %H:%M:%S").output() {
        if output.status.success() {
            if let Ok(date) = String::from_utf8(output.stdout) {
                return date.trim().to_string();
            }
        }
    }

    // Fallback
    "unknown date".to_string()
}

fn print_summary(
    findings: &[model::ScoredFinding],
    hostname: &str,
    asset_ctx: AssetCriticality,
    output_path: &str,
) {
    let total = findings.len();
    let mut critical = 0;
    let mut high = 0;
    let mut medium = 0;
    let mut low = 0;
    let mut passed = 0;
    let mut cumulative_risk = 0.0;

    for sf in findings {
        if sf.finding.passed {
            passed += 1;
        } else {
            cumulative_risk += sf.risk;
            match sf.risk_band() {
                "critical" => critical += 1,
                "high" => high += 1,
                "medium" => medium += 1,
                "low" => low += 1,
                _ => {}
            }
        }
    }

    println!("confrisk v0.1 — skan zakończony");
    println!("host: {} | profil: {}", hostname, asset_ctx.label());
    println!(
        "findingi: {} (critical: {}, high: {}, medium: {}, low: {}, passed: {})",
        total, critical, high, medium, low, passed
    );
    println!("skumulowane ryzyko: {:.1}", cumulative_risk);
    println!("raport: {}", output_path);
}
