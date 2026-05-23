# Linux Security Scanner

This repository contains the **confrisk** security assessment framework.

## What is confrisk?

**confrisk** is a generic, config-driven security assessment framework for Linux systems that:

- 🔍 Scans system configurations, dependencies, and network exposure
- 📊 Scores findings using contextual risk assessment (not just severity)
- 🎯 Prioritizes remediation by effort-adjusted risk
- 📝 Generates detailed HTML reports with explainable scoring
- 🔌 Integrates external scanners without code changes
- ⚙️ Fully configurable via JSON files

## Quick Start

```bash
cd confrisk

# Build
cargo build --release

# Run scan
cargo run -- --asset production --out report.html

# Open report
open report.html  # macOS
xdg-open report.html  # Linux
```

## Documentation

- **[confrisk/README.md](confrisk/README.md)** — Main documentation
- **[confrisk/CONFIG_SYSTEM.md](confrisk/CONFIG_SYSTEM.md)** — Configuration system (NEW!)
- **[confrisk/CONFIG_QUICKSTART.md](confrisk/CONFIG_QUICKSTART.md)** — Quick start guide
- **[confrisk/DEMO_RESULTS.md](confrisk/DEMO_RESULTS.md)** — Demo execution results

## Features

### v0.1 — Original Implementation
- 8 hardcoded security checks (SSH, file permissions, kernel hardening)
- Risk-based scoring model
- Contextual priority calculation
- HTML report generation
- Docker demo with vulnerable container

### v0.2 — Config-Driven Architecture (NEW!)
- 🎯 **12 generic issue categories** (PRIVILEGES, DEPENDENCIES, OPEN_PORTS, LOGS, SECRETS, etc.)
- ⚙️ **Configurable risk model** — Adjust weights in JSON, no code changes
- 📝 **JSON-based checks** — Add security checks by creating JSON files
- 🔌 **Plugin system** — Integrate Trivy, Lynis, Gitleaks, OSV-Scanner
- 🚫 **User-defined rules** — Block vulnerable packages, flag dangerous ports
- 🔧 **5 detection types** — config_directive, file_permission, command_output, file_exists, custom

## Directory Structure

```
linux-security-scanner/
├── .gitignore
├── README.md (this file)
└── confrisk/
    ├── config/              ← NEW: JSON configuration files
    │   ├── categories.json
    │   ├── scoring.json
    │   ├── checks/
    │   ├── plugins/
    │   └── rules/
    ├── src/
    │   ├── main.rs
    │   ├── model.rs
    │   ├── checks.rs
    │   ├── report.rs
    │   └── config.rs        ← NEW: Config loader
    ├── demo/
    │   ├── Dockerfile.vulnerable
    │   └── docker-compose.yml
    └── README.md
```

## Example: Add Your Own Check

Create `confrisk/config/checks/my-check.json`:

```json
{
  "id": "CUSTOM-001",
  "name": "My security check",
  "category": "COMPLIANCE",
  "severity": "high",
  "detection": {
    "type": "file_permission",
    "file": "/etc/my-config",
    "max_mode": "0644"
  },
  "remediation": {
    "summary": "Fix permissions",
    "steps": ["chmod 644 /etc/my-config"]
  }
}
```

Run `confrisk` — it automatically loads the new check!

## Demo

A complete demo with a vulnerable Docker container is available:

```bash
cd confrisk/demo

# Build and run
docker compose run --rm confrisk-demo

# View report
open out/report.html
```

The demo shows how the **same vulnerabilities** get **different risk scores** based on asset criticality (dev vs production vs crown-jewel).

## Contributing

This is an educational/research project demonstrating:
- Contextual risk assessment
- Config-driven architecture
- Plugin-based scanner integration
- Explainable security scoring

## License

Educational/Research project.

---

**Version:** 0.2.0-alpha
**Last Updated:** May 23, 2026
