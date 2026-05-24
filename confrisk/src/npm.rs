/// NPM dependency scanner and vulnerability checker

use crate::config::{BlockedPackage, Config};
use crate::model::{AssetCriticality, Exposure, Finding, Severity};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Debug, Deserialize)]
pub struct PackageJson {
    pub name: Option<String>,
    pub version: Option<String>,
    #[serde(default)]
    pub dependencies: HashMap<String, String>,
    #[serde(default)]
    #[serde(rename = "devDependencies")]
    pub dev_dependencies: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct PackageLock {
    pub name: Option<String>,
    pub version: Option<String>,
    #[serde(default)]
    pub packages: HashMap<String, PackageLockPackage>,
}

#[derive(Debug, Deserialize)]
pub struct PackageLockPackage {
    pub version: Option<String>,
    #[serde(default)]
    pub dev: bool,
}

#[derive(Debug, Deserialize)]
pub struct NpmAuditResult {
    pub vulnerabilities: HashMap<String, NpmVulnerability>,
    pub metadata: NpmAuditMetadata,
}

#[derive(Debug, Deserialize)]
pub struct NpmVulnerability {
    pub severity: String,
    pub via: Vec<NpmVia>,
    pub effects: Vec<String>,
    pub range: String,
    pub nodes: Vec<String>,
    #[serde(rename = "fixAvailable")]
    pub fix_available: serde_json::Value,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum NpmVia {
    String(String),
    Object(NpmViaObject),
}

#[derive(Debug, Deserialize)]
pub struct NpmViaObject {
    pub source: Option<u64>,
    pub name: Option<String>,
    pub dependency: Option<String>,
    pub title: Option<String>,
    pub url: Option<String>,
    pub severity: Option<String>,
    pub cwe: Option<Vec<String>>,
    pub cvss: Option<serde_json::Value>,
    pub range: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct NpmAuditMetadata {
    pub vulnerabilities: NpmVulnerabilityCounts,
    pub dependencies: NpmDependencyCounts,
}

#[derive(Debug, Deserialize)]
pub struct NpmVulnerabilityCounts {
    #[serde(default)]
    pub info: u32,
    #[serde(default)]
    pub low: u32,
    #[serde(default)]
    pub moderate: u32,
    #[serde(default)]
    pub high: u32,
    #[serde(default)]
    pub critical: u32,
    #[serde(default)]
    pub total: u32,
}

#[derive(Debug, Deserialize)]
pub struct NpmDependencyCounts {
    #[serde(default)]
    pub prod: u32,
    #[serde(default)]
    pub dev: u32,
    #[serde(default)]
    pub optional: u32,
    #[serde(default)]
    pub peer: u32,
    #[serde(default)]
    pub peerOptional: u32,
    #[serde(default)]
    pub total: u32,
}

/// NPM Scanner
pub struct NpmScanner {
    config: Config,
    project_path: String,
}

impl NpmScanner {
    pub fn new(config: Config, project_path: String) -> Self {
        Self {
            config,
            project_path,
        }
    }

    /// Run all npm security checks
    pub fn scan(&self) -> Vec<Finding> {
        let mut findings = Vec::new();

        // Check if package.json exists
        let pkg_path = Path::new(&self.project_path).join("package.json");
        if !pkg_path.exists() {
            findings.push(Finding {
                id: "NPM-000".to_string(),
                title: "No package.json found".to_string(),
                description: format!(
                    "No package.json found in {}. This doesn't appear to be an npm project.",
                    self.project_path
                ),
                severity: Severity::Info,
                exposure: Exposure::Local,
                confidence: 1.0,
                effort: 0.0,
                remediation: "Run 'npm init' to create a package.json".to_string(),
                evidence: format!("Checked path: {}", pkg_path.display()),
                passed: false,
            });
            return findings;
        }

        // Check for blocklisted packages
        findings.extend(self.check_blocklist());

        // Run npm audit if available
        findings.extend(self.run_npm_audit());

        // Check for outdated packages
        findings.extend(self.check_outdated());

        findings
    }

    /// Check for blocklisted packages
    fn check_blocklist(&self) -> Vec<Finding> {
        let mut findings = Vec::new();

        let pkg_json = match self.read_package_json() {
            Some(pkg) => pkg,
            None => return findings,
        };

        // Combine all dependencies
        let mut all_deps = pkg_json.dependencies.clone();
        all_deps.extend(pkg_json.dev_dependencies.clone());

        // Check against blocklist
        for blocked in &self.config.dependencies_rules.blocklist.packages {
            if blocked.ecosystem != "npm" {
                continue;
            }

            for (dep_name, dep_version) in &all_deps {
                if dep_name != &blocked.name {
                    continue;
                }

                // Check version pattern if specified
                let matches = if blocked.version_pattern.is_empty() {
                    true
                } else {
                    // Simple version matching (could use regex crate for more complex)
                    dep_version.contains(&blocked.name) || self.version_matches(dep_version, &blocked.version_pattern)
                };

                if matches {
                    let severity = match blocked.severity.as_str() {
                        "critical" => Severity::Critical,
                        "high" => Severity::High,
                        "medium" => Severity::Medium,
                        "low" => Severity::Low,
                        _ => Severity::Medium,
                    };

                    findings.push(Finding {
                        id: format!("NPM-BLOCKED-{}", blocked.name.to_uppercase()),
                        title: format!("Blocked package: {}", blocked.name),
                        description: blocked.reason.clone(),
                        severity,
                        exposure: Exposure::Local,
                        confidence: 0.99,
                        effort: 2.0,
                        remediation: format!(
                            "Replace '{}' with '{}'. Current version: {}",
                            blocked.name, blocked.alternative, dep_version
                        ),
                        evidence: format!(
                            "Found {} version {} in {}",
                            dep_name,
                            dep_version,
                            if pkg_json.dependencies.contains_key(dep_name) {
                                "dependencies"
                            } else {
                                "devDependencies"
                            }
                        ),
                        passed: false,
                    });
                }
            }
        }

        findings
    }

    /// Run npm audit
    fn run_npm_audit(&self) -> Vec<Finding> {
        let mut findings = Vec::new();

        let output = Command::new("npm")
            .arg("audit")
            .arg("--json")
            .current_dir(&self.project_path)
            .output();

        let output = match output {
            Ok(o) => o,
            Err(_) => {
                findings.push(Finding {
                    id: "NPM-AUDIT-ERROR".to_string(),
                    title: "npm audit not available".to_string(),
                    description: "Could not run npm audit. Is npm installed?".to_string(),
                    severity: Severity::Info,
                    exposure: Exposure::Local,
                    confidence: 0.5,
                    effort: 0.0,
                    remediation: "Install npm or run manually: npm audit".to_string(),
                    evidence: "npm command not found".to_string(),
                    passed: true,
                });
                return findings;
            }
        };

        let stdout = String::from_utf8_lossy(&output.stdout);
        let audit: Result<NpmAuditResult, _> = serde_json::from_str(&stdout);

        if let Ok(audit) = audit {
            // Summary finding
            if audit.metadata.vulnerabilities.total > 0 {
                let severity = if audit.metadata.vulnerabilities.critical > 0 {
                    Severity::Critical
                } else if audit.metadata.vulnerabilities.high > 0 {
                    Severity::High
                } else if audit.metadata.vulnerabilities.moderate > 0 {
                    Severity::Medium
                } else {
                    Severity::Low
                };

                findings.push(Finding {
                    id: "NPM-AUDIT-SUMMARY".to_string(),
                    title: format!(
                        "{} vulnerabilities found by npm audit",
                        audit.metadata.vulnerabilities.total
                    ),
                    description: format!(
                        "npm audit found {} total vulnerabilities: {} critical, {} high, {} moderate, {} low",
                        audit.metadata.vulnerabilities.total,
                        audit.metadata.vulnerabilities.critical,
                        audit.metadata.vulnerabilities.high,
                        audit.metadata.vulnerabilities.moderate,
                        audit.metadata.vulnerabilities.low
                    ),
                    severity,
                    exposure: Exposure::InternetFacing,
                    confidence: 0.95,
                    effort: 2.0,
                    remediation: "Run 'npm audit fix' to automatically fix vulnerabilities".to_string(),
                    evidence: format!(
                        "Critical: {}, High: {}, Moderate: {}, Low: {}",
                        audit.metadata.vulnerabilities.critical,
                        audit.metadata.vulnerabilities.high,
                        audit.metadata.vulnerabilities.moderate,
                        audit.metadata.vulnerabilities.low
                    ),
                    passed: false,
                });
            }

            // Individual vulnerabilities
            for (pkg_name, vuln) in audit.vulnerabilities.iter().take(10) {
                let severity = match vuln.severity.to_lowercase().as_str() {
                    "critical" => Severity::Critical,
                    "high" => Severity::High,
                    "moderate" => Severity::Medium,
                    "low" => Severity::Low,
                    _ => Severity::Info,
                };

                let title = vuln.via.iter()
                    .filter_map(|v| {
                        if let NpmVia::Object(obj) = v {
                            obj.title.clone()
                        } else {
                            None
                        }
                    })
                    .next()
                    .unwrap_or_else(|| format!("Vulnerability in {}", pkg_name));

                findings.push(Finding {
                    id: format!("NPM-AUDIT-{}", pkg_name.to_uppercase().replace('/', "-")),
                    title,
                    description: format!(
                        "Package '{}' has a {} severity vulnerability. Version range: {}",
                        pkg_name, vuln.severity, vuln.range
                    ),
                    severity,
                    exposure: Exposure::InternetFacing,
                    confidence: 0.95,
                    effort: 2.0,
                    remediation: if vuln.fix_available.is_boolean() && !vuln.fix_available.as_bool().unwrap_or(false) {
                        format!("No automatic fix available. Check for manual update of {}", pkg_name)
                    } else {
                        "Run 'npm audit fix' or update package manually".to_string()
                    },
                    evidence: format!(
                        "Package: {}, Range: {}, Severity: {}",
                        pkg_name, vuln.range, vuln.severity
                    ),
                    passed: false,
                });
            }
        }

        findings
    }

    /// Check for outdated packages
    fn check_outdated(&self) -> Vec<Finding> {
        let mut findings = Vec::new();

        let output = Command::new("npm")
            .arg("outdated")
            .arg("--json")
            .current_dir(&self.project_path)
            .output();

        if let Ok(output) = output {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if let Ok(outdated) = serde_json::from_str::<HashMap<String, serde_json::Value>>(&stdout) {
                if !outdated.is_empty() && outdated.len() > 5 {
                    findings.push(Finding {
                        id: "NPM-OUTDATED".to_string(),
                        title: format!("{} outdated packages", outdated.len()),
                        description: format!(
                            "Found {} outdated packages. Outdated packages may contain security vulnerabilities.",
                            outdated.len()
                        ),
                        severity: Severity::Low,
                        exposure: Exposure::Local,
                        confidence: 0.8,
                        effort: 3.0,
                        remediation: "Run 'npm outdated' to see details, then 'npm update'".to_string(),
                        evidence: format!("{} packages need updating", outdated.len()),
                        passed: false,
                    });
                }
            }
        }

        findings
    }

    /// Read package.json
    fn read_package_json(&self) -> Option<PackageJson> {
        let path = Path::new(&self.project_path).join("package.json");
        let content = fs::read_to_string(path).ok()?;
        serde_json::from_str(&content).ok()
    }

    /// Simple version pattern matching
    fn version_matches(&self, version: &str, pattern: &str) -> bool {
        // Remove ^ and ~ prefixes
        let clean_version = version.trim_start_matches('^').trim_start_matches('~');

        // Simple pattern matching - could be enhanced with regex
        if pattern.contains('*') {
            let pattern_parts: Vec<&str> = pattern.split('.').collect();
            let version_parts: Vec<&str> = clean_version.split('.').collect();

            for (i, pattern_part) in pattern_parts.iter().enumerate() {
                if *pattern_part == "*" {
                    continue;
                }
                if i >= version_parts.len() || version_parts[i] != *pattern_part {
                    return false;
                }
            }
            true
        } else {
            clean_version.starts_with(pattern.trim_start_matches('^'))
        }
    }
}
