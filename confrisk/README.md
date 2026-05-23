# confrisk — Linux Security Configuration Scanner

**confrisk** is a risk-based security configuration scanner for Linux systems. It scans system configurations, scores findings based on contextual risk, and prioritizes remediation efforts.

## Key Features

- **Contextual Risk Scoring** — Not just severity, but actual risk based on asset criticality and exposure
- **Prioritization** — Automatically sorts findings by priority (risk ÷ effort) to identify "quick wins"
- **Explainability** — Every score includes a breakdown showing how it was calculated
- **Zero Dependencies** — Pure Rust with stdlib only, single static binary
- **Security Console UI** — Dark-themed HTML reports with collapsible findings
- **8 Real Security Checks** — SSH config, file permissions, kernel hardening, network settings

## Risk Assessment Model

confrisk uses a **hybrid risk model** that considers multiple factors:

```
risk = severity × asset_criticality × exposure × confidence
priority = risk ÷ effort
```

### Factors

| Factor | Range | Description |
|--------|-------|-------------|
| **severity** | 1.0-10.0 | Base severity (Info/Low/Medium/High/Critical) |
| **asset_criticality** | 0.5-1.3 | How critical the asset is (dev/internal/production/crown-jewel) |
| **exposure** | 0.7-1.25 | Attack surface (local/adjacent/internet-facing) |
| **confidence** | 0.0-1.0 | Certainty that it's not a false positive |
| **effort** | 1.0-5.0 | Remediation effort (trivial → architectural change) |

The **priority** score surfaces high-risk, low-effort fixes first — turning a "list of findings" into an actionable plan.

## Installation

### From Source

```bash
# Clone or download the project
cd confrisk

# Build
cargo build --release

# The binary will be at: target/release/confrisk
```

### Static Linux Binary (for deployment)

```bash
# Add musl target
rustup target add x86_64-unknown-linux-musl

# Build static binary
cargo build --release --target x86_64-unknown-linux-musl

# Install system-wide
sudo install -m 755 target/x86_64-unknown-linux-musl/release/confrisk /usr/local/bin/confrisk
```

## Usage

```bash
confrisk [OPTIONS]

OPTIONS:
    --asset <PROFILE>    Asset criticality profile
                         Values: dev, internal, production, crown-jewel
                         Default: production

    --out <FILE>         Output HTML report path
                         Default: report.html

    --help, -h           Show help message
```

### Examples

```bash
# Scan a development machine
confrisk --asset dev --out dev-report.html

# Scan a production server (requires root for full scan)
sudo confrisk --asset production --out /var/log/security-scan.html

# Scan a critical asset
sudo confrisk --asset crown-jewel --out crown-jewel-scan.html
```

### Understanding Output

```
confrisk v0.1 — skan zakończony
host: web-server-01 | profil: production
findingi: 8 (critical: 2, high: 3, medium: 1, low: 1, passed: 1)
skumulowane ryzyko: 41.7
raport: report.html
```

Open the HTML report in a browser to see:
- Overall security posture
- Risk statistics
- Detailed findings with evidence and remediation steps
- Score breakdowns for auditability

## Security Checks

| ID | Check | Severity | File/Source |
|----|-------|----------|-------------|
| SSH-001 | Root login disabled | High | /etc/ssh/sshd_config |
| SSH-002 | Password auth disabled | Medium | /etc/ssh/sshd_config |
| FILE-001 | /etc/passwd permissions | Medium | /etc/passwd |
| FILE-002 | /etc/shadow not world-readable | Critical | /etc/shadow |
| FILE-003 | No world-writable /etc files | High | /etc/* scan |
| KRNL-001 | ASLR fully enabled | Medium | /proc/sys/kernel/randomize_va_space |
| NET-001 | IP forwarding disabled | Low | /proc/sys/net/ipv4/ip_forward |
| CRON-001 | Crontab permissions secure | High | /etc/crontab |

## Demo: Vulnerable Container

A complete demo is available in the `demo/` directory, featuring:
- Intentionally vulnerable Docker container
- Multiple security misconfigurations
- Side-by-side comparison of different asset profiles

See [demo/README.md](demo/README.md) for instructions.

## Permissions

Some checks require **root access** to read protected files (e.g., `/etc/shadow`). Without root:
- Checks that can't access files will report `passed=true` with low confidence
- No errors or crashes — the tool degrades gracefully

For full scan coverage, run as root:
```bash
sudo confrisk --asset production --out /tmp/full-scan.html
```

## Architecture

```
confrisk/
├── src/
│   ├── main.rs      # CLI, pipeline orchestration
│   ├── model.rs     # Risk model, scoring engine, types
│   ├── checks.rs    # 8 security check implementations
│   └── report.rs    # HTML report generator
└── demo/
    ├── Dockerfile.vulnerable  # Vulnerable container
    ├── docker-compose.yml     # Multi-profile demo
    └── README.md             # Demo instructions
```

**Pipeline:** `collect → normalize → score → prioritize → report`

The modular design allows easy addition of new checks or integration with other scanners (OpenSCAP, Trivy, etc.) by implementing the `Finding` interface.

## Roadmap

- [ ] JSON export for CI/CD integration
- [ ] Exit code based on risk threshold (fail builds)
- [ ] More checks (sudoers, umask, mount options)
- [ ] Configurable risk model weights (via TOML)
- [ ] OpenSCAP/Trivy adapter (import findings from other tools)

## Project Context

This is a university project demonstrating:
- **Risk assessment model** — Hybrid scoring with multiple factors
- **Scanner + prioritization** — Beyond simple vulnerability listing
- **Working prototype** — Real checks, real risk calculation
- **Demonstrable results** — Vulnerable container showing contextual risk

The goal is to show how risk scoring should account for **where** and **what** is vulnerable, not just **how severe** the vulnerability is in isolation.

## License

Educational/research project.
