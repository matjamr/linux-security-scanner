/// Configuration loader for JSON-based check definitions, plugins, and rules

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Environment variable that points to the confrisk configuration directory.
///
/// This is the single source of truth for where the scanner (npm, gradle, and
/// any future generic ecosystem) looks for its JSON rules, checks, plugins and
/// scoring definitions. Set it once and every binary picks it up:
///
/// ```sh
/// export CONFRISK_CONFIG_DIR=/etc/confrisk
/// ```
pub const CONFIG_DIR_ENV: &str = "CONFRISK_CONFIG_DIR";

/// A marker file that must exist inside a directory for it to be considered a
/// valid confrisk config root. Used when probing the standard fallback paths.
const CONFIG_MARKER: &str = "categories.json";

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
    /// Resolve the configuration directory that all scanners should read from.
    ///
    /// Resolution order (first match wins):
    ///   1. `explicit` — an explicit override (e.g. a CLI `--config` flag).
    ///      Used verbatim; an error is returned if it is not a valid config dir.
    ///   2. `$CONFRISK_CONFIG_DIR` — the canonical environment variable.
    ///      Used verbatim; an error is returned if it is not a valid config dir.
    ///   3. Standard Linux locations, probed in order, first existing wins:
    ///        - `$XDG_CONFIG_HOME/confrisk`
    ///        - `$HOME/.config/confrisk`
    ///        - `/etc/confrisk`
    ///        - `/usr/local/share/confrisk/config`
    ///        - `/usr/share/confrisk/config`
    ///   4. `./config` — development fallback (current working directory).
    ///
    /// The intent is that operators set `CONFRISK_CONFIG_DIR` once and every
    /// ecosystem scanner (npm, gradle, generic) reads from the same place.
    pub fn resolve_dir(explicit: Option<&str>) -> Result<String, String> {
        // 1. Explicit override (CLI flag). Honor it strictly so typos surface.
        if let Some(dir) = explicit {
            if Self::is_config_dir(dir) {
                return Ok(dir.to_string());
            }
            return Err(format!(
                "config directory '{}' (from --config) does not contain {}",
                dir, CONFIG_MARKER
            ));
        }

        // 2. The canonical environment variable. Honor it strictly too.
        if let Ok(dir) = std::env::var(CONFIG_DIR_ENV) {
            if !dir.is_empty() {
                if Self::is_config_dir(&dir) {
                    return Ok(dir);
                }
                return Err(format!(
                    "{}='{}' does not contain {} — set it to a valid confrisk config directory",
                    CONFIG_DIR_ENV, dir, CONFIG_MARKER
                ));
            }
        }

        // 3. Standard Linux locations.
        for candidate in Self::standard_dirs() {
            if Self::is_config_dir(&candidate) {
                return Ok(candidate.to_string_lossy().into_owned());
            }
        }

        // 4. Development fallback.
        if Self::is_config_dir("config") {
            return Ok("config".to_string());
        }

        Err(format!(
            "could not locate a confrisk config directory. Set {} to the directory \
             containing {} (e.g. /etc/confrisk), or pass --config <PATH>.",
            CONFIG_DIR_ENV, CONFIG_MARKER
        ))
    }

    /// Load configuration using [`Config::resolve_dir`] to locate the directory.
    ///
    /// This is the entry point every binary should use so that the
    /// `CONFRISK_CONFIG_DIR` environment variable is honored uniformly.
    pub fn load_resolved(explicit: Option<&str>) -> Result<Self, String> {
        let dir = Self::resolve_dir(explicit)?;
        Self::load(&dir)
    }

    /// The ordered list of standard config locations probed on Linux.
    fn standard_dirs() -> Vec<PathBuf> {
        let mut dirs = Vec::new();

        if let Ok(xdg) = std::env::var("XDG_CONFIG_HOME") {
            if !xdg.is_empty() {
                dirs.push(Path::new(&xdg).join("confrisk"));
            }
        }
        if let Ok(home) = std::env::var("HOME") {
            if !home.is_empty() {
                dirs.push(Path::new(&home).join(".config").join("confrisk"));
            }
        }
        dirs.push(PathBuf::from("/etc/confrisk"));
        dirs.push(PathBuf::from("/usr/local/share/confrisk/config"));
        dirs.push(PathBuf::from("/usr/share/confrisk/config"));

        dirs
    }

    /// A directory is a valid config root if it exists and holds the marker file.
    fn is_config_dir<P: AsRef<Path>>(dir: P) -> bool {
        dir.as_ref().join(CONFIG_MARKER).is_file()
    }

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

#[cfg(test)]
mod resolve_tests {
    use super::*;

    /// `--config` takes precedence and is honored verbatim when valid.
    #[test]
    fn explicit_flag_wins() {
        // The repo's own ./config is a valid directory when tests run from the crate root.
        let dir = Config::resolve_dir(Some("config")).expect("config/ should resolve");
        assert_eq!(dir, "config");
    }

    /// An explicit but invalid `--config` path fails loudly instead of falling through.
    #[test]
    fn invalid_explicit_flag_errors() {
        let err = Config::resolve_dir(Some("/definitely/not/here")).unwrap_err();
        assert!(err.contains("--config"));
    }

    /// The marker-file check only accepts directories that actually hold config.
    #[test]
    fn marker_detection() {
        assert!(Config::is_config_dir("config"));
        assert!(!Config::is_config_dir("/tmp"));
    }
}
