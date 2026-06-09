/// HTML report generator — clean, light, plain-styled report

use crate::model::{AssetCriticality, ScoredFinding};

/// Escape HTML special characters
fn esc(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

/// Polish label for a risk band / passed state
fn band_label(band: &str) -> &'static str {
    match band {
        "critical" => "Krytyczne",
        "high" => "Wysokie",
        "medium" => "Średnie",
        "low" => "Niskie",
        "passed" => "Zaliczone",
        _ => "Informacja",
    }
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

    // Overall posture — a single plain status word with a muted colour.
    let (posture, posture_color) = if critical_count >= 2 {
        ("Zagrożony", "#b3261e")
    } else if critical_count >= 1 || high_count >= 3 {
        ("Wymaga uwagi", "#b54708")
    } else if high_count >= 1 || medium_count >= 2 {
        ("Do poprawy", "#946800")
    } else if failed_count > 0 {
        ("Stabilny z uwagami", "#1f5fa8")
    } else {
        ("Bezpieczny", "#2e7d32")
    };

    // Findings: open the ones that did not pass so issues are visible.
    let findings_html: String = findings
        .iter()
        .map(|sf| render_finding(sf, ctx, !sf.finding.passed))
        .collect();

    format!(
        r#"<!DOCTYPE html>
<html lang="pl">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Raport bezpieczeństwa — {hostname}</title>
    <style>
        :root {{
            --bg: #f5f6f7;
            --card: #ffffff;
            --border: #e2e4e8;
            --text: #20242a;
            --muted: #6b7280;
        }}
        * {{ box-sizing: border-box; }}
        body {{
            margin: 0;
            background: var(--bg);
            color: var(--text);
            font-family: -apple-system, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
            font-size: 15px;
            line-height: 1.55;
        }}
        .wrap {{ max-width: 820px; margin: 0 auto; padding: 32px 20px 64px; }}

        header h1 {{ font-size: 22px; font-weight: 600; margin: 0 0 6px; }}
        .meta {{ color: var(--muted); font-size: 14px; margin: 0; }}
        .meta b {{ color: var(--text); font-weight: 600; }}

        .panel {{
            background: var(--card);
            border: 1px solid var(--border);
            border-radius: 8px;
            padding: 16px 18px;
            margin-top: 20px;
        }}

        .status {{ display: flex; align-items: center; gap: 10px; margin-bottom: 4px; }}
        .status .dot {{ width: 10px; height: 10px; border-radius: 50%; flex: none; }}
        .status .word {{ font-size: 17px; font-weight: 600; }}
        .status .sub {{ color: var(--muted); font-size: 14px; }}

        .counts {{ display: flex; flex-wrap: wrap; gap: 18px; margin-top: 14px; }}
        .counts div {{ font-size: 14px; color: var(--muted); }}
        .counts b {{ display: block; font-size: 20px; color: var(--text); font-weight: 600; }}

        h2 {{ font-size: 16px; font-weight: 600; margin: 28px 0 10px; }}

        details.finding {{
            background: var(--card);
            border: 1px solid var(--border);
            border-radius: 8px;
            margin-bottom: 8px;
        }}
        details.finding > summary {{
            list-style: none;
            cursor: pointer;
            padding: 12px 14px;
            display: flex;
            align-items: center;
            gap: 10px;
        }}
        details.finding > summary::-webkit-details-marker {{ display: none; }}
        summary .chev {{ color: var(--muted); transition: transform .15s; flex: none; }}
        details[open] summary .chev {{ transform: rotate(90deg); }}
        summary .title {{ font-weight: 600; flex: 1; }}
        summary .prio {{ color: var(--muted); font-size: 13px; white-space: nowrap; }}

        .tag {{
            font-size: 12px;
            font-weight: 600;
            padding: 2px 9px;
            border-radius: 20px;
            white-space: nowrap;
            flex: none;
        }}
        .tag.critical {{ background: #fbeae8; color: #b3261e; }}
        .tag.high     {{ background: #fdf0e6; color: #b54708; }}
        .tag.medium   {{ background: #fbf4e0; color: #8a6400; }}
        .tag.low      {{ background: #e8f0fb; color: #1f5fa8; }}
        .tag.info     {{ background: #eef0f2; color: #5b6470; }}
        .tag.passed   {{ background: #e7f3e8; color: #2e7d32; }}

        .body {{ padding: 0 14px 14px 14px; border-top: 1px solid var(--border); }}
        .body p.desc {{ margin: 12px 0 14px; }}
        dl {{ margin: 0; }}
        dt {{ color: var(--muted); font-size: 13px; margin-top: 12px; }}
        dd {{ margin: 3px 0 0; }}
        dd.mono {{ font-family: ui-monospace, "SF Mono", Menlo, Consolas, monospace; font-size: 13px; color: #374151; }}

        details.note {{ margin-top: 24px; color: var(--muted); font-size: 14px; }}
        details.note summary {{ cursor: pointer; }}
        details.note code {{ font-family: ui-monospace, Menlo, Consolas, monospace; font-size: 13px; }}

        footer {{ margin-top: 36px; color: var(--muted); font-size: 13px; }}
    </style>
</head>
<body>
    <div class="wrap">
        <header>
            <h1>Raport bezpieczeństwa</h1>
            <p class="meta">Host <b>{hostname}</b> &middot; profil zasobu <b>{asset_label}</b> &middot; data {scan_date} &middot; kontroli: <b>{total}</b></p>
        </header>

        <div class="panel">
            <div class="status">
                <span class="dot" style="background:{posture_color}"></span>
                <span class="word">{posture}</span>
                <span class="sub">— nieudanych kontroli {failed_count} z {total}, skumulowane ryzyko {cumulative_risk:.1}</span>
            </div>
            <div class="counts">
                <div><b>{critical_count}</b> krytyczne</div>
                <div><b>{high_count}</b> wysokie</div>
                <div><b>{medium_count}</b> średnie</div>
                <div><b>{low_count}</b> niskie</div>
                <div><b>{passed_count}</b> zaliczone</div>
            </div>
        </div>

        <h2>Wyniki kontroli</h2>
        {findings_html}

        <details class="note">
            <summary>Jak liczone jest ryzyko?</summary>
            <p>Ryzyko = dotkliwość &times; krytyczność zasobu &times; ekspozycja &times; pewność. Priorytet = ryzyko &divide; nakład naprawy. Wyniki są sortowane malejąco po priorytecie, a każdy zawiera rozbicie oceny na czynniki.</p>
        </details>

        <footer>confrisk — ocena konfiguracji systemu</footer>
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

/// Render a single finding as a collapsible row
fn render_finding(sf: &ScoredFinding, ctx: AssetCriticality, open: bool) -> String {
    let band = if sf.finding.passed { "passed" } else { sf.risk_band() };
    let open_attr = if open { " open" } else { "" };

    let score_section = if sf.finding.passed {
        String::new()
    } else {
        format!(
            "<dt>Dlaczego taki wynik</dt><dd class=\"mono\">{}</dd>",
            esc(&sf.explanation(ctx))
        )
    };

    format!(
        r#"<details class="finding"{open_attr}>
    <summary>
        <span class="chev">&#9656;</span>
        <span class="tag {band}">{band_label}</span>
        <span class="title">{title}</span>
        <span class="prio">priorytet {priority:.1}</span>
    </summary>
    <div class="body">
        <p class="desc">{description}</p>
        <dl>
            <dt>Identyfikator</dt><dd class="mono">{id}</dd>
            <dt>Co wykryto</dt><dd>{evidence}</dd>
            {score_section}
            <dt>Nakład naprawy</dt><dd>{effort:.1} / 5</dd>
            <dt>Rekomendacja</dt><dd>{remediation}</dd>
        </dl>
    </div>
</details>
"#,
        open_attr = open_attr,
        band = band,
        band_label = band_label(band),
        id = esc(&sf.finding.id),
        title = esc(&sf.finding.title),
        priority = sf.priority,
        description = esc(&sf.finding.description),
        evidence = esc(&sf.finding.evidence),
        score_section = score_section,
        effort = sf.finding.effort,
        remediation = esc(&sf.finding.remediation),
    )
}
