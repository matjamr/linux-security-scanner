/// HTML report generator with security console theme

use crate::model::{AssetCriticality, ScoredFinding};

/// Escape HTML special characters
fn esc(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

/// Generate complete HTML report
pub fn render(
    findings: &[ScoredFinding],
    ctx: AssetCriticality,
    hostname: &str,
    scan_date: &str,
) -> String {
    let total = findings.len();
    let mut critical_count = 0;
    let mut high_count = 0;
    let mut medium_count = 0;
    let mut low_count = 0;
    let mut passed_count = 0;
    let mut cumulative_risk = 0.0;

    for sf in findings {
        if sf.finding.passed {
            passed_count += 1;
        } else {
            cumulative_risk += sf.risk;
            match sf.risk_band() {
                "critical" => critical_count += 1,
                "high" => high_count += 1,
                "medium" => medium_count += 1,
                "low" => low_count += 1,
                _ => {}
            }
        }
    }

    let failed_count = total - passed_count;

    // Determine overall posture
    let (posture, posture_color) = if critical_count >= 2 {
        ("ZAGROŻONY", "#ef4444")
    } else if critical_count >= 1 || high_count >= 3 {
        ("WYMAGA UWAGI", "#f97316")
    } else if high_count >= 1 || medium_count >= 2 {
        ("DO POPRAWY", "#eab308")
    } else if failed_count > 0 {
        ("STABILNY Z UWAGAMI", "#3b82f6")
    } else {
        ("BEZPIECZNY", "#22c55e")
    };

    // Generate findings HTML
    let findings_html: String = findings
        .iter()
        .enumerate()
        .map(|(idx, sf)| render_finding(sf, ctx, idx < 3))
        .collect();

    format!(
        r#"<!DOCTYPE html>
<html lang="pl">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>confrisk — raport bezpieczeństwa</title>
    <style>
        * {{
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }}

        body {{
            background: #0a0a0a;
            color: #e5e7eb;
            font-family: 'JetBrains Mono', 'Fira Code', 'Consolas', monospace;
            font-size: 14px;
            line-height: 1.6;
            padding: 2rem;
        }}

        .container {{
            max-width: 1200px;
            margin: 0 auto;
        }}

        header {{
            background: #1a1a1a;
            padding: 1.5rem;
            border-radius: 8px;
            border-left: 4px solid #3b82f6;
            margin-bottom: 2rem;
        }}

        h1 {{
            font-size: 1.75rem;
            color: #60a5fa;
            margin-bottom: 0.5rem;
        }}

        .meta {{
            color: #9ca3af;
            font-size: 0.9rem;
        }}

        .meta span {{
            margin-right: 1.5rem;
        }}

        .posture-banner {{
            background: linear-gradient(135deg, #1a1a1a 0%, #2a2a2a 100%);
            padding: 2rem;
            border-radius: 8px;
            border-left: 6px solid {posture_color};
            margin-bottom: 2rem;
            text-align: center;
        }}

        .posture-status {{
            font-size: 2rem;
            font-weight: bold;
            color: {posture_color};
            margin-bottom: 0.5rem;
        }}

        .posture-detail {{
            color: #9ca3af;
        }}

        .stats {{
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
            gap: 1rem;
            margin-bottom: 2rem;
        }}

        .stat-card {{
            background: #1a1a1a;
            padding: 1.25rem;
            border-radius: 8px;
            border-left: 4px solid;
        }}

        .stat-card.critical {{ border-color: #ef4444; }}
        .stat-card.high {{ border-color: #f97316; }}
        .stat-card.medium {{ border-color: #eab308; }}
        .stat-card.low {{ border-color: #3b82f6; }}
        .stat-card.passed {{ border-color: #22c55e; }}

        .stat-label {{
            font-size: 0.85rem;
            color: #9ca3af;
            text-transform: uppercase;
            letter-spacing: 0.05em;
        }}

        .stat-value {{
            font-size: 2rem;
            font-weight: bold;
            margin-top: 0.5rem;
        }}

        .stat-card.critical .stat-value {{ color: #ef4444; }}
        .stat-card.high .stat-value {{ color: #f97316; }}
        .stat-card.medium .stat-value {{ color: #eab308; }}
        .stat-card.low .stat-value {{ color: #3b82f6; }}
        .stat-card.passed .stat-value {{ color: #22c55e; }}

        .model-box {{
            background: #1a1a1a;
            padding: 1.5rem;
            border-radius: 8px;
            border-left: 4px solid #8b5cf6;
            margin-bottom: 2rem;
        }}

        .model-box h2 {{
            color: #a78bfa;
            margin-bottom: 1rem;
            font-size: 1.25rem;
        }}

        .model-box code {{
            background: #0a0a0a;
            padding: 0.75rem;
            border-radius: 4px;
            display: block;
            color: #fbbf24;
            margin-bottom: 0.5rem;
        }}

        .model-box p {{
            color: #9ca3af;
            font-size: 0.9rem;
            margin-top: 0.5rem;
        }}

        .findings {{
            margin-top: 2rem;
        }}

        .findings h2 {{
            color: #60a5fa;
            margin-bottom: 1.5rem;
            font-size: 1.5rem;
        }}

        .finding {{
            background: #1a1a1a;
            margin-bottom: 1rem;
            border-radius: 8px;
            border-left: 6px solid;
            overflow: hidden;
        }}

        .finding.critical {{ border-color: #ef4444; }}
        .finding.high {{ border-color: #f97316; }}
        .finding.medium {{ border-color: #eab308; }}
        .finding.low {{ border-color: #3b82f6; }}
        .finding.info {{ border-color: #6b7280; }}
        .finding.passed {{ border-color: #22c55e; }}

        details summary {{
            padding: 1.25rem;
            cursor: pointer;
            user-select: none;
            list-style: none;
        }}

        details summary::-webkit-details-marker {{
            display: none;
        }}

        details summary::before {{
            content: '▶ ';
            margin-right: 0.5rem;
            transition: transform 0.2s;
            display: inline-block;
        }}

        details[open] summary::before {{
            transform: rotate(90deg);
        }}

        .finding-header {{
            display: flex;
            justify-content: space-between;
            align-items: center;
        }}

        .finding-title {{
            font-weight: bold;
            color: #e5e7eb;
        }}

        .finding-id {{
            color: #9ca3af;
            font-size: 0.85rem;
            margin-right: 0.75rem;
        }}

        .finding-priority {{
            background: #0a0a0a;
            padding: 0.25rem 0.75rem;
            border-radius: 4px;
            font-size: 0.85rem;
            color: #fbbf24;
        }}

        .finding-body {{
            padding: 0 1.25rem 1.25rem 1.25rem;
            border-top: 1px solid #2a2a2a;
        }}

        .finding-section {{
            margin-top: 1rem;
        }}

        .finding-section-title {{
            color: #9ca3af;
            font-size: 0.85rem;
            text-transform: uppercase;
            letter-spacing: 0.05em;
            margin-bottom: 0.5rem;
        }}

        .finding-section-content {{
            color: #d1d5db;
            background: #0a0a0a;
            padding: 0.75rem;
            border-radius: 4px;
        }}

        .evidence {{
            color: #fbbf24;
            font-style: italic;
        }}

        .explanation {{
            color: #a78bfa;
            font-family: monospace;
        }}

        footer {{
            margin-top: 3rem;
            padding-top: 2rem;
            border-top: 1px solid #2a2a2a;
            text-align: center;
            color: #6b7280;
            font-size: 0.85rem;
        }}
    </style>
</head>
<body>
    <div class="container">
        <header>
            <h1>confrisk — skan bezpieczeństwa</h1>
            <div class="meta">
                <span><strong>host:</strong> {hostname}</span>
                <span><strong>profil zasobu:</strong> {asset_label}</span>
                <span><strong>data skanu:</strong> {scan_date}</span>
                <span><strong>kontroli:</strong> {total}</span>
            </div>
        </header>

        <div class="posture-banner">
            <div class="posture-status">{posture}</div>
            <div class="posture-detail">
                Skumulowane ryzyko: {cumulative_risk:.1} | Kontroli nieudanych: {failed_count}/{total}
            </div>
        </div>

        <div class="stats">
            <div class="stat-card critical">
                <div class="stat-label">Critical</div>
                <div class="stat-value">{critical_count}</div>
            </div>
            <div class="stat-card high">
                <div class="stat-label">High</div>
                <div class="stat-value">{high_count}</div>
            </div>
            <div class="stat-card medium">
                <div class="stat-label">Medium</div>
                <div class="stat-value">{medium_count}</div>
            </div>
            <div class="stat-card low">
                <div class="stat-label">Low</div>
                <div class="stat-value">{low_count}</div>
            </div>
            <div class="stat-card passed">
                <div class="stat-label">Passed</div>
                <div class="stat-value">{passed_count}</div>
            </div>
        </div>

        <div class="model-box">
            <h2>Model Oceny Ryzyka</h2>
            <code>risk = severity × asset_criticality × exposure × confidence</code>
            <code>priority = risk ÷ effort</code>
            <p>
                Findingi są sortowane według priorytetu (malejąco). Wyższy priorytet = większe ryzyko przy niższym nakładzie remediacji.
                Każdy finding zawiera rozbicie score na czynniki dla pełnej audytowalności.
            </p>
        </div>

        <div class="findings">
            <h2>Wyniki kontroli (sortowane po priorytecie)</h2>
            {findings_html}
        </div>

        <footer>
            confrisk v0.1 — framework oceny ryzyka konfiguracji Linux
        </footer>
    </div>
</body>
</html>"#,
        hostname = esc(hostname),
        asset_label = esc(ctx.label()),
        scan_date = esc(scan_date),
        total = total,
        critical_count = critical_count,
        high_count = high_count,
        medium_count = medium_count,
        low_count = low_count,
        passed_count = passed_count,
        failed_count = failed_count,
        cumulative_risk = cumulative_risk,
        posture = posture,
        posture_color = posture_color,
        findings_html = findings_html,
    )
}

/// Render a single finding as HTML details element
fn render_finding(sf: &ScoredFinding, ctx: AssetCriticality, open: bool) -> String {
    let band = if sf.finding.passed {
        "passed"
    } else {
        sf.risk_band()
    };

    let open_attr = if open { " open" } else { "" };

    let status_icon = if sf.finding.passed { "✓" } else { "✗" };

    format!(
        r#"<details class="finding {band}"{open_attr}>
    <summary>
        <div class="finding-header">
            <div>
                <span class="finding-id">{status_icon} {id}</span>
                <span class="finding-title">{title}</span>
            </div>
            <span class="finding-priority">priority: {priority:.2}</span>
        </div>
    </summary>
    <div class="finding-body">
        <div class="finding-section">
            <div class="finding-section-title">Opis</div>
            <div class="finding-section-content">{description}</div>
        </div>
        <div class="finding-section">
            <div class="finding-section-title">Wykryto (Evidence)</div>
            <div class="finding-section-content evidence">{evidence}</div>
        </div>
        {score_section}
        <div class="finding-section">
            <div class="finding-section-title">Nakład remediacji</div>
            <div class="finding-section-content">{effort:.1} / 5.0</div>
        </div>
        <div class="finding-section">
            <div class="finding-section-title">Rekomendacja</div>
            <div class="finding-section-content">{remediation}</div>
        </div>
    </div>
</details>
"#,
        band = band,
        open_attr = open_attr,
        status_icon = status_icon,
        id = esc(&sf.finding.id),
        title = esc(&sf.finding.title),
        priority = sf.priority,
        description = esc(&sf.finding.description),
        evidence = esc(&sf.finding.evidence),
        score_section = if !sf.finding.passed {
            format!(
                r#"<div class="finding-section">
            <div class="finding-section-title">Uzasadnienie Score</div>
            <div class="finding-section-content explanation">{}</div>
        </div>"#,
                esc(&sf.explanation(ctx))
            )
        } else {
            String::new()
        },
        effort = sf.finding.effort,
        remediation = esc(&sf.finding.remediation),
    )
}
