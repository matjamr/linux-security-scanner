/// Risk assessment model for Linux configuration findings

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Severity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

impl Severity {
    pub fn weight(&self) -> f64 {
        match self {
            Severity::Info => 1.0,
            Severity::Low => 3.0,
            Severity::Medium => 5.5,
            Severity::High => 8.0,
            Severity::Critical => 10.0,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Severity::Info => "info",
            Severity::Low => "low",
            Severity::Medium => "medium",
            Severity::High => "high",
            Severity::Critical => "critical",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AssetCriticality {
    Dev,
    Internal,
    Production,
    CrownJewel,
}

impl AssetCriticality {
    pub fn multiplier(&self) -> f64 {
        match self {
            AssetCriticality::Dev => 0.5,
            AssetCriticality::Internal => 0.8,
            AssetCriticality::Production => 1.1,
            AssetCriticality::CrownJewel => 1.3,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            AssetCriticality::Dev => "dev",
            AssetCriticality::Internal => "internal",
            AssetCriticality::Production => "production",
            AssetCriticality::CrownJewel => "crown-jewel",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "dev" => Some(AssetCriticality::Dev),
            "internal" => Some(AssetCriticality::Internal),
            "production" | "prod" => Some(AssetCriticality::Production),
            "crown-jewel" | "crownjewel" => Some(AssetCriticality::CrownJewel),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Exposure {
    Local,
    AdjacentNetwork,
    InternetFacing,
}

impl Exposure {
    pub fn multiplier(&self) -> f64 {
        match self {
            Exposure::Local => 0.7,
            Exposure::AdjacentNetwork => 0.95,
            Exposure::InternetFacing => 1.25,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Exposure::Local => "local",
            Exposure::AdjacentNetwork => "adjacent",
            Exposure::InternetFacing => "internet-facing",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Finding {
    pub id: String,
    pub title: String,
    pub description: String,
    pub severity: Severity,
    pub exposure: Exposure,
    pub confidence: f64,       // 0.0–1.0
    pub effort: f64,           // 1.0–5.0
    pub remediation: String,
    pub evidence: String,
    pub passed: bool,
}

#[derive(Debug, Clone)]
pub struct ScoredFinding {
    pub finding: Finding,
    pub risk: f64,
    pub priority: f64,
}

impl ScoredFinding {
    pub fn explanation(&self, ctx: AssetCriticality) -> String {
        if self.finding.passed {
            return "PASSED — no risk".to_string();
        }

        let sev = self.finding.severity.weight();
        let asset = ctx.multiplier();
        let expo = self.finding.exposure.multiplier();
        let conf = self.finding.confidence;

        format!(
            "{:.1} (sev:{}) × {:.2} (asset:{}) × {:.2} (expo:{}) × {:.2} (conf) = {:.2}",
            sev,
            self.finding.severity.label(),
            asset,
            ctx.label(),
            expo,
            self.finding.exposure.label(),
            conf,
            self.risk
        )
    }

    pub fn risk_band(&self) -> &'static str {
        if self.risk >= 9.0 {
            "critical"
        } else if self.risk >= 6.0 {
            "high"
        } else if self.risk >= 3.5 {
            "medium"
        } else if self.risk >= 1.5 {
            "low"
        } else {
            "info"
        }
    }
}

/// Score all findings and sort by priority (descending)
pub fn score_all(findings: Vec<Finding>, ctx: AssetCriticality) -> Vec<ScoredFinding> {
    let mut scored: Vec<ScoredFinding> = findings
        .into_iter()
        .map(|f| {
            let risk = if f.passed {
                0.0
            } else {
                f.severity.weight() * ctx.multiplier() * f.exposure.multiplier() * f.confidence
            };

            let priority = if f.effort > 0.0 {
                risk / f.effort
            } else {
                risk
            };

            ScoredFinding {
                finding: f,
                risk,
                priority,
            }
        })
        .collect();

    // Sort by priority (descending), then by risk (descending) for ties
    scored.sort_by(|a, b| {
        b.priority.partial_cmp(&a.priority)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| b.risk.partial_cmp(&a.risk).unwrap_or(std::cmp::Ordering::Equal))
    });

    scored
}
