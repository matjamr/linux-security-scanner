/// Generator raportu HTML

use crate::model::{AssetCriticality, ScoredFinding};

/// Escapowanie znakow HTML
fn esc(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

/// Wyciaga komende z remediacji w konwencji "Run: <komenda>"
fn fix_command(remediation: &str) -> Option<String> {
    let rest = remediation.trim().strip_prefix("Run:")?.trim_start();
    let cmd = rest.split(';').next().unwrap_or(rest);
    let cmd = cmd.split(" or ").next().unwrap_or(cmd).trim();
    if cmd.is_empty() {
        None
    } else {
        Some(cmd.to_string())
    }
}

/// Czy remediacja opisuje realny krok naprawczy
fn is_actionable(remediation: &str) -> bool {
    let r = remediation.trim();
    !r.is_empty() && !r.starts_with("N/A")
}

/// Etykieta pasma ryzyka
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

/// Raport HTML dla skanu systemu
pub fn render(
    findings: &[ScoredFinding],
    ctx: AssetCriticality,
    hostname: &str,
    scan_date: &str,
) -> String {
    render_inner(findings, ctx, "Host", hostname, scan_date)
}

/// Raport HTML dla skanu projektu (npm/gradle)
pub fn render_project(
    findings: &[ScoredFinding],
    ctx: AssetCriticality,
    project: &str,
    scan_date: &str,
) -> String {
    render_inner(findings, ctx, "Projekt", project, scan_date)
}

fn render_inner(
    findings: &[ScoredFinding],
    ctx: AssetCriticality,
    subject_label: &str,
    subject: &str,
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

    let findings_html: String = findings
        .iter()
        .map(|sf| render_finding(sf, ctx, !sf.finding.passed))
        .collect();

    let actionable_count = findings
        .iter()
        .filter(|sf| !sf.finding.passed && is_actionable(&sf.finding.remediation))
        .count();
    let fix_all_button = if actionable_count > 0 {
        format!(
            r#"<div class="fixall-row"><button id="fixall">Pobierz skrypt naprawczy (fix.sh) — {n}</button><span class="note-text">Wszystkie kroki naprawcze: gotowe komendy + kroki ręczne jako komentarze. Przeglądarka nie zmienia systemu.</span></div>"#,
            n = actionable_count
        )
    } else {
        String::new()
    };

    format!(
        r#"<!DOCTYPE html>
<html lang="pl">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Raport bezpieczeństwa — {subject}</title>
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

        button.fix, #fixall {{
            font: inherit;
            font-size: 13px;
            cursor: pointer;
            border: 1px solid #cdd0d5;
            background: #fff;
            color: var(--text);
            padding: 6px 12px;
            border-radius: 6px;
        }}
        button.fix {{ border-color: #bcd9c4; color: #256a2a; }}
        button.fix:hover, #fixall:hover {{ background: #f0f1f3; }}
        .fix-row {{ margin-top: 14px; display: flex; align-items: center; gap: 10px; }}
        .fix-hint {{ font-size: 12px; color: #2e7d32; }}
        .fixall-row {{ display: flex; align-items: center; gap: 12px; margin: 4px 0 14px; }}
        .fixall-row .note-text {{ font-size: 12px; color: var(--muted); }}

        footer {{ margin-top: 36px; color: var(--muted); font-size: 13px; }}
    </style>
</head>
<body>
    <div class="wrap">
        <header>
            <h1>Raport bezpieczeństwa</h1>
            <p class="meta">{subject_label} <b>{subject}</b> &middot; profil zasobu <b>{asset_label}</b> &middot; data {scan_date} &middot; kontroli: <b>{total}</b></p>
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
        {fix_all_button}
        {findings_html}

        <details class="note">
            <summary>Jak liczone jest ryzyko?</summary>
            <p>Ryzyko = dotkliwość &times; krytyczność zasobu &times; ekspozycja &times; pewność. Priorytet = ryzyko &divide; nakład naprawy. Wyniki są sortowane malejąco po priorytecie, a każdy zawiera rozbicie oceny na czynniki.</p>
        </details>

        <footer>confrisk — ocena konfiguracji systemu</footer>
    </div>
    <script>
        function copyText(t) {{
            return new Promise(function (res) {{
                function fallback() {{
                    var ta = document.createElement('textarea');
                    ta.value = t; ta.style.position = 'fixed'; ta.style.opacity = '0';
                    document.body.appendChild(ta); ta.focus(); ta.select();
                    try {{ document.execCommand('copy'); }} catch (e) {{}}
                    document.body.removeChild(ta); res();
                }}
                if (navigator.clipboard && navigator.clipboard.writeText) {{
                    navigator.clipboard.writeText(t).then(function () {{ res(); }}, fallback);
                }} else {{
                    fallback();
                }}
            }});
        }}
        document.querySelectorAll('button.fix').forEach(function (b) {{
            b.addEventListener('click', function () {{
                copyText(b.getAttribute('data-cmd')).then(function () {{
                    var h = b.parentElement.querySelector('.fix-hint');
                    if (h) {{ h.textContent = 'Skopiowano — wklej w terminalu'; setTimeout(function () {{ h.textContent = ''; }}, 2500); }}
                }});
            }});
        }});
        var fa = document.getElementById('fixall');
        if (fa) {{
            fa.addEventListener('click', function () {{
                var lines = [
                    '#!/bin/sh',
                    '# Skrypt naprawczy wygenerowany przez confrisk.',
                    '# PRZEJRZYJ przed uruchomieniem — część poleceń wymaga uprawnień root (sudo).',
                    '# Wiersze komentarza (#) to kroki ręczne — brak bezpiecznej komendy jednolinijkowej.',
                    ''
                ];
                document.querySelectorAll('details.finding[data-rem]').forEach(function (d) {{
                    lines.push('# [' + (d.getAttribute('data-id') || '') + '] ' + (d.getAttribute('data-title') || ''));
                    var cmd = d.getAttribute('data-cmd');
                    if (cmd) {{
                        lines.push(cmd);
                    }} else {{
                        lines.push('# RĘCZNIE: ' + d.getAttribute('data-rem'));
                    }}
                    lines.push('');
                }});
                var blob = new Blob([lines.join('\n')], {{ type: 'text/x-shellscript' }});
                var a = document.createElement('a');
                a.href = URL.createObjectURL(blob); a.download = 'fix.sh';
                document.body.appendChild(a); a.click(); document.body.removeChild(a);
                URL.revokeObjectURL(a.href);
            }});
        }}
    </script>
</body>
</html>"#,
        subject_label = esc(subject_label),
        subject = esc(subject),
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
        fix_all_button = fix_all_button,
        findings_html = findings_html,
    )
}

/// Pojedynczy wynik jako rozwijana sekcja
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

    let fix_button = if sf.finding.passed {
        String::new()
    } else {
        match fix_command(&sf.finding.remediation) {
            Some(cmd) => format!(
                r#"<div class="fix-row"><button class="fix" data-cmd="{}">Napraw automatycznie</button><span class="fix-hint"></span></div>"#,
                esc(&cmd)
            ),
            None => String::new(),
        }
    };

    let data_attrs = if !sf.finding.passed && is_actionable(&sf.finding.remediation) {
        let cmd_attr = match fix_command(&sf.finding.remediation) {
            Some(cmd) => format!(r#" data-cmd="{}""#, esc(&cmd)),
            None => String::new(),
        };
        format!(
            r#" data-id="{}" data-title="{}" data-rem="{}"{}"#,
            esc(&sf.finding.id),
            esc(&sf.finding.title),
            esc(sf.finding.remediation.trim()),
            cmd_attr
        )
    } else {
        String::new()
    };

    format!(
        r#"<details class="finding"{data_attrs}{open_attr}>
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
        {fix_button}
    </div>
</details>
"#,
        open_attr = open_attr,
        data_attrs = data_attrs,
        band = band,
        band_label = band_label(band),
        id = esc(&sf.finding.id),
        title = esc(&sf.finding.title),
        priority = sf.priority,
        description = esc(&sf.finding.description),
        evidence = esc(&sf.finding.evidence),
        score_section = score_section,
        fix_button = fix_button,
        effort = sf.finding.effort,
        remediation = esc(&sf.finding.remediation),
    )
}

#[cfg(test)]
mod fix_tests {
    use super::fix_command;

    #[test]
    fn extracts_simple_command() {
        assert_eq!(fix_command("Run: chmod 644 /etc/passwd").as_deref(), Some("chmod 644 /etc/passwd"));
    }

    #[test]
    fn takes_first_of_alternatives() {
        assert_eq!(
            fix_command("Run: chmod 640 /etc/shadow or chmod 600 /etc/shadow").as_deref(),
            Some("chmod 640 /etc/shadow")
        );
    }

    #[test]
    fn stops_before_prose_after_semicolon() {
        assert_eq!(
            fix_command("Run: echo 2 | sudo tee /proc/sys/kernel/randomize_va_space; add '...' to /etc/sysctl.conf").as_deref(),
            Some("echo 2 | sudo tee /proc/sys/kernel/randomize_va_space")
        );
    }

    #[test]
    fn none_without_run_prefix() {
        assert_eq!(fix_command("Set 'PermitRootLogin no' in /etc/ssh/sshd_config"), None);
        assert_eq!(fix_command("N/A"), None);
    }
}
