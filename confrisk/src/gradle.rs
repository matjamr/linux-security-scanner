/// Gradle dependency scanner and vulnerability checker

use crate::config::Config;
use crate::model::{Exposure, Finding, Severity};
use crate::scanner::Scanner;
use regex::Regex;
use std::fs;
use std::path::Path;

/// Gradle Scanner
pub struct GradleScanner {
    config: Config,
    project_path: String,
}

impl GradleScanner {
    pub fn new(config: Config, project_path: String) -> Self {
        Self {
            config,
            project_path,
        }
    }
}

impl Scanner for GradleScanner {
    /// Run all Gradle security checks
    fn scan(&self) -> Vec<Finding> {
        let mut findings = Vec::new();

        // Check if build.gradle or build.gradle.kts exists
        let gradle_path = Path::new(&self.project_path).join("build.gradle");
        let gradle_kts_path = Path::new(&self.project_path).join("build.gradle.kts");

        if !gradle_path.exists() && !gradle_kts_path.exists() {
            findings.push(Finding {
                id: "GRADLE-000".to_string(),
                title: "No Gradle build file found".to_string(),
                description: format!(
                    "No build.gradle or build.gradle.kts found in {}. This doesn't appear to be a Gradle project.",
                    self.project_path
                ),
                severity: Severity::Info,
                exposure: Exposure::Local,
                confidence: 1.0,
                effort: 0.0,
                remediation: "Initialize a Gradle project with 'gradle init'".to_string(),
                evidence: format!(
                    "Checked paths: {}, {}",
                    gradle_path.display(),
                    gradle_kts_path.display()
                ),
                passed: false,
            });
            return findings;
        }

        // Parse and check dependencies
        findings.extend(self.check_blocklist());

        findings
    }
}

impl GradleScanner {
    /// Check for blocklisted packages
    fn check_blocklist(&self) -> Vec<Finding> {
        let mut findings = Vec::new();

        // Parse build.gradle
        let gradle_path = Path::new(&self.project_path).join("build.gradle");
        if gradle_path.exists() {
            if let Ok(content) = fs::read_to_string(&gradle_path) {
                findings.extend(self.parse_gradle_dependencies(&content, "build.gradle"));
            }
        }

        // Parse build.gradle.kts
        let gradle_kts_path = Path::new(&self.project_path).join("build.gradle.kts");
        if gradle_kts_path.exists() {
            if let Ok(content) = fs::read_to_string(&gradle_kts_path) {
                findings.extend(self.parse_gradle_dependencies(&content, "build.gradle.kts"));
            }
        }

        findings
    }

    /// Parse Gradle dependencies from build file content
    fn parse_gradle_dependencies(&self, content: &str, filename: &str) -> Vec<Finding> {
        let mut findings = Vec::new();

        // Regex patterns for Gradle dependencies
        // Matches: implementation 'group:artifact:version'
        // Matches: implementation("group:artifact:version")
        // Matches: implementation group: 'group', name: 'artifact', version: 'version'
        let patterns = vec![
            // Simple format: implementation 'group:artifact:version'
            Regex::new(r#"(?:implementation|compile|api|runtimeOnly|testImplementation|compileOnly)\s+['"]([^:'"]+):([^:'"]+):([^'"]+)['"]"#).unwrap(),
            // Kotlin DSL: implementation("group:artifact:version")
            Regex::new(r#"(?:implementation|compile|api|runtimeOnly|testImplementation|compileOnly)\s*\(\s*"([^:"]+):([^:"]+):([^"]+)"\s*\)"#).unwrap(),
            // Map notation: implementation group: 'group', name: 'artifact', version: 'version'
            // This is more complex, we'll handle it separately
        ];

        for pattern in patterns {
            for cap in pattern.captures_iter(content) {
                let group = cap.get(1).map_or("", |m| m.as_str());
                let artifact = cap.get(2).map_or("", |m| m.as_str());
                let version = cap.get(3).map_or("", |m| m.as_str());

                // Check against blocklist (Maven ecosystem works for Gradle)
                for blocked in &self.config.dependencies_rules.blocklist.packages {
                    if blocked.ecosystem != "maven" {
                        continue;
                    }

                    // Check artifact name (sometimes with group prefix)
                    let full_name = format!("{}.{}", group, artifact);
                    let matches_name = artifact == blocked.name
                        || full_name.contains(&blocked.name)
                        || artifact.contains(&blocked.name);

                    if !matches_name {
                        continue;
                    }

                    // Check version pattern if specified
                    let matches_version = if blocked.version_pattern.is_empty() {
                        true
                    } else {
                        self.version_matches(version, &blocked.version_pattern)
                    };

                    if matches_version {
                        let severity = match blocked.severity.as_str() {
                            "critical" => Severity::Critical,
                            "high" => Severity::High,
                            "medium" => Severity::Medium,
                            "low" => Severity::Low,
                            _ => Severity::Medium,
                        };

                        findings.push(Finding {
                            id: format!("GRADLE-BLOCKED-{}", blocked.name.to_uppercase().replace('-', "_")),
                            title: format!("Blocked dependency: {}", blocked.name),
                            description: blocked.reason.clone(),
                            severity,
                            exposure: Exposure::InternetFacing,
                            confidence: 0.99,
                            effort: 2.0,
                            remediation: format!(
                                "Replace '{}:{}:{}' with '{}'. See: {}",
                                group, artifact, version, blocked.alternative, filename
                            ),
                            evidence: format!(
                                "Found {}:{}:{} in {}",
                                group, artifact, version, filename
                            ),
                            passed: false,
                        });
                    }
                }
            }
        }

        // Also check map notation: group: 'x', name: 'y', version: 'z'
        findings.extend(self.parse_map_notation_dependencies(content, filename));

        findings
    }

    /// Parse map notation dependencies
    fn parse_map_notation_dependencies(&self, content: &str, filename: &str) -> Vec<Finding> {
        let mut findings = Vec::new();

        // Pattern for map notation
        // Matches: implementation group: 'org.springframework', name: 'spring-core', version: '5.3.5'
        let map_pattern = Regex::new(
            r#"(?:implementation|compile|api|runtimeOnly|testImplementation|compileOnly)\s+group:\s*['"]([^'"]+)['"],\s*name:\s*['"]([^'"]+)['"],\s*version:\s*['"]([^'"]+)['"]"#
        ).unwrap();

        for cap in map_pattern.captures_iter(content) {
            let group = cap.get(1).map_or("", |m| m.as_str());
            let artifact = cap.get(2).map_or("", |m| m.as_str());
            let version = cap.get(3).map_or("", |m| m.as_str());

            // Check against blocklist
            for blocked in &self.config.dependencies_rules.blocklist.packages {
                if blocked.ecosystem != "maven" {
                    continue;
                }

                let full_name = format!("{}.{}", group, artifact);
                let matches_name = artifact == blocked.name
                    || full_name.contains(&blocked.name)
                    || artifact.contains(&blocked.name);

                if !matches_name {
                    continue;
                }

                let matches_version = if blocked.version_pattern.is_empty() {
                    true
                } else {
                    self.version_matches(version, &blocked.version_pattern)
                };

                if matches_version {
                    let severity = match blocked.severity.as_str() {
                        "critical" => Severity::Critical,
                        "high" => Severity::High,
                        "medium" => Severity::Medium,
                        "low" => Severity::Low,
                        _ => Severity::Medium,
                    };

                    findings.push(Finding {
                        id: format!("GRADLE-BLOCKED-{}", blocked.name.to_uppercase().replace('-', "_")),
                        title: format!("Blocked dependency: {}", blocked.name),
                        description: blocked.reason.clone(),
                        severity,
                        exposure: Exposure::InternetFacing,
                        confidence: 0.99,
                        effort: 2.0,
                        remediation: format!(
                            "Update to {}. Current: {}:{}:{}",
                            blocked.alternative, group, artifact, version
                        ),
                        evidence: format!(
                            "Found {}:{}:{} in {}",
                            group, artifact, version, filename
                        ),
                        passed: false,
                    });
                }
            }
        }

        findings
    }

    /// Version pattern matching using regex
    fn version_matches(&self, version: &str, pattern: &str) -> bool {
        // Try regex matching
        if let Ok(re) = Regex::new(pattern) {
            return re.is_match(version);
        }

        // Fallback to simple wildcard matching
        if pattern.contains('*') {
            let pattern_parts: Vec<&str> = pattern.split('.').collect();
            let version_parts: Vec<&str> = version.split('.').collect();

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
            version.starts_with(pattern)
        }
    }
}
