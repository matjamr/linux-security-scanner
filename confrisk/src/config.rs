/// Configuration loader for JSON-based check definitions, plugins, and rules

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Category definition
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Category {
    pub id: String,
    pub name: String,
    pub description: String,
    pub color: String,
}

/// Categories configuration
#[derive(Debug, Deserialize, Serialize)]
pub struct CategoriesConfig {
    pub categories: Vec<Category>,
}

/// Scoring configuration
#[derive(Debug, Deserialize, Serialize)]
pub struct ScoringConfig {
    pub version: String,
    pub model: ModelFormulas,
    pub severity: HashMap<String, f64>,
    pub asset_criticality: HashMap<String, f64>,
    pub exposure: HashMap<String, f64>,
    pub risk_bands: HashMap<String, f64>,
    pub effort_multipliers: HashMap<String, f64>,
    pub confidence_adjustments: HashMap<String, f64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ModelFormulas {
    pub formula: String,
    pub priority_formula: String,
}

/// Check detection configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum Detection {
    #[serde(rename = "config_directive")]
    ConfigDirective {
        file: String,
        directive: String,
        expected: String,
        #[serde(default)]
        fail_on_missing: bool,
        #[serde(default = "default_confidence")]
        missing_confidence: f64,
    },

    #[serde(rename = "file_permission")]
    FilePermission {
        file: String,
        check: String,
        max_mode: String,
    },

    #[serde(rename = "command_output")]
    CommandOutput {
        command: String,
        pattern: String,
        expected: String,
    },

    #[serde(rename = "file_exists")]
    FileExists {
        file: String,
        should_exist: bool,
    },

    #[serde(rename = "custom")]
    Custom {
        script: String,
    },
}

fn default_confidence() -> f64 {
    0.7
}

/// Remediation steps
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Remediation {
    pub summary: String,
    pub steps: Vec<String>,
    #[serde(default)]
    pub references: Vec<String>,
}

/// Check definition
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CheckConfig {
    pub id: String,
    pub name: String,
    pub category: String,
    pub description: String,
    pub severity: String,
    pub exposure: String,
    pub confidence: f64,
    pub effort: String,
    #[serde(default = "default_true")]
    pub enabled: bool,
    pub detection: Detection,
    pub remediation: Remediation,
    #[serde(default)]
    pub tags: Vec<String>,
}

fn default_true() -> bool {
    true
}

/// Plugin scan configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PluginScan {
    pub name: String,
    pub command: String,
    pub timeout_seconds: u64,
    pub output_format: String,
    pub categories: Vec<String>,
}

/// Plugin mapping configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PluginMapping {
    pub severity: HashMap<String, String>,
    pub confidence: PluginConfidence,
    pub exposure: PluginExposure,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PluginConfidence {
    pub default: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PluginExposure {
    pub default: String,
    #[serde(default)]
    pub overrides: HashMap<String, String>,
}

/// Plugin parser configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PluginParser {
    #[serde(rename = "type")]
    pub parser_type: String,
    #[serde(default)]
    pub results_path: String,
    #[serde(default)]
    pub fields: HashMap<String, String>,
    #[serde(default)]
    pub patterns: Vec<ParserPattern>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ParserPattern {
    pub regex: String,
    pub groups: HashMap<String, u32>,
    pub severity: String,
}

/// Plugin installation info
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PluginInstallation {
    pub check_command: String,
    pub install_url: String,
    pub install_instructions: String,
}

/// Plugin configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PluginConfig {
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub required: bool,
    pub installation: PluginInstallation,
    pub scans: Vec<PluginScan>,
    pub mapping: PluginMapping,
    pub parser: PluginParser,
}

/// Dependency blocklist entry
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BlockedPackage {
    pub name: String,
    pub ecosystem: String,
    #[serde(default)]
    pub version_pattern: String,
    pub reason: String,
    pub severity: String,
    pub alternative: String,
}

/// Port rule
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PortRule {
    pub port: u16,
    pub protocol: String,
    pub name: String,
    pub severity: String,
    pub reason: String,
    pub remediation: String,
    pub exposure: String,
}

/// Dependencies rules configuration
#[derive(Debug, Deserialize, Serialize)]
pub struct DependenciesRules {
    pub version: String,
    pub description: String,
    pub blocklist: BlocklistSection,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BlocklistSection {
    pub description: String,
    pub packages: Vec<BlockedPackage>,
}

/// Ports rules configuration
#[derive(Debug, Deserialize, Serialize)]
pub struct PortsRules {
    pub version: String,
    pub description: String,
    pub dangerous_ports: Vec<PortRule>,
}

/// Main configuration holder
#[derive(Debug)]
pub struct Config {
    pub categories: CategoriesConfig,
    pub scoring: ScoringConfig,
    pub checks: Vec<CheckConfig>,
    pub plugins: Vec<PluginConfig>,
    pub dependencies_rules: DependenciesRules,
    pub ports_rules: PortsRules,
}

impl Config {
    /// Load all configuration from the config directory
    pub fn load(config_dir: &str) -> Result<Self, String> {
        let config_path = Path::new(config_dir);

        // Load categories
        let categories = Self::load_json::<CategoriesConfig>(
            &config_path.join("categories.json")
        )?;

        // Load scoring
        let scoring = Self::load_json::<ScoringConfig>(
            &config_path.join("scoring.json")
        )?;

        // Load checks
        let checks_dir = config_path.join("checks");
        let mut checks = Vec::new();
        if checks_dir.exists() {
            for entry in fs::read_dir(&checks_dir).map_err(|e| e.to_string())? {
                let entry = entry.map_err(|e| e.to_string())?;
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("json") {
                    let check = Self::load_json::<CheckConfig>(&path)?;
                    if check.enabled {
                        checks.push(check);
                    }
                }
            }
        }

        // Load plugins
        let plugins_dir = config_path.join("plugins");
        let mut plugins = Vec::new();
        if plugins_dir.exists() {
            for entry in fs::read_dir(&plugins_dir).map_err(|e| e.to_string())? {
                let entry = entry.map_err(|e| e.to_string())?;
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("json") {
                    let plugin = Self::load_json::<PluginConfig>(&path)?;
                    plugins.push(plugin);
                }
            }
        }

        // Load dependency rules
        let dependencies_rules = Self::load_json::<DependenciesRules>(
            &config_path.join("rules/dependencies.json")
        )?;

        // Load port rules
        let ports_rules = Self::load_json::<PortsRules>(
            &config_path.join("rules/ports.json")
        )?;

        Ok(Config {
            categories,
            scoring,
            checks,
            plugins,
            dependencies_rules,
            ports_rules,
        })
    }

    /// Load a JSON file
    fn load_json<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T, String> {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read {:?}: {}", path, e))?;

        serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse {:?}: {}", path, e))
    }

    /// Get severity weight from scoring config
    pub fn get_severity_weight(&self, severity: &str) -> f64 {
        self.scoring.severity.get(severity)
            .copied()
            .unwrap_or(5.0)
    }

    /// Get exposure multiplier from scoring config
    pub fn get_exposure_multiplier(&self, exposure: &str) -> f64 {
        self.scoring.exposure.get(exposure)
            .copied()
            .unwrap_or(1.0)
    }

    /// Get effort multiplier from scoring config
    pub fn get_effort_multiplier(&self, effort: &str) -> f64 {
        self.scoring.effort_multipliers.get(effort)
            .copied()
            .unwrap_or(2.0)
    }
}
