# confrisk-npm — NPM Dependency Security Scanner

A dedicated security scanner for npm projects that integrates seamlessly with git hooks (husky), CI/CD pipelines, and development workflows.

## What is confrisk-npm?

**confrisk-npm** scans your npm projects for:
- 🚫 **Blocked packages** — Vulnerable or deprecated packages from your blocklist
- 🔍 **npm audit vulnerabilities** — Known CVEs in dependencies
- 📦 **Outdated packages** — Dependencies that need updating
- 🎯 **Risk-based prioritization** — Contextual scoring, not just severity

Unlike `npm audit`, confrisk-npm provides:
- **Configurable blocklist** — Ban specific packages/versions company-wide
- **Contextual risk scoring** — Different weights for dev vs production
- **Priority calculation** — Surface high-risk, low-effort fixes first
- **CI/CD integration** — Fail builds on threshold violations
- **Git hooks** — Prevent commits with vulnerabilities

## Installation

### From Source

```bash
cd confrisk
cargo build --release --bin confrisk-npm

# Install globally
sudo cp target/release/confrisk-npm /usr/local/bin/

# Or add to PATH
export PATH=$PATH:$(pwd)/target/release
```

### Binary Release (Coming Soon)

```bash
# Download pre-built binary
curl -L https://github.com/you/confrisk/releases/latest/download/confrisk-npm-linux -o confrisk-npm
chmod +x confrisk-npm
sudo mv confrisk-npm /usr/local/bin/
```

## Quick Start

### Basic Scan

```bash
# Scan current directory
confrisk-npm

# Scan specific project
confrisk-npm --path /path/to/project

# JSON output
confrisk-npm --format json > report.json
```

### Example Output

```
┌────────────────────────────────────────────────────────────┐
│  confrisk-npm — NPM Security Scan                          │
└────────────────────────────────────────────────────────────┘

Project: .
Checks: 8

┌─ Summary ─────────────────────────────────────────────────┐
│ Critical:   2                                              │
│ High:       3                                              │
│ Medium:     1                                              │
│ Low:        1                                              │
│ Passed:     1                                              │
└───────────────────────────────────────────────────────────┘

┌─ Issues Found (sorted by priority) ───────────────────────┐

🔴 [NPM-BLOCKED-LODASH] Blocked package: lodash (priority: 8.2)
   Prototype pollution vulnerabilities < 4.17.21
   Evidence: Found lodash version ^4.17.20 in dependencies
   Fix: Replace 'lodash' with 'lodash >= 4.17.21'

🟠 [NPM-AUDIT-EXPRESS] Vulnerability in express (priority: 6.1)
   Package 'express' has a high severity vulnerability
   Evidence: Package: express, Range: <4.19.0, Severity: high
   Fix: Run 'npm audit fix' or update package manually

✅ No critical issues found!
```

## Usage

### Command-Line Options

```bash
confrisk-npm [OPTIONS]

OPTIONS:
    -p, --path <PATH>        Path to npm project (default: .)
    -a, --asset <PROFILE>    Asset criticality: dev, internal, production, crown-jewel
                             (default: production)
    -f, --format <FORMAT>    Output format: text, json (default: text)
    -c, --config <PATH>      Config directory path (default: config)
    --fail-on <LEVEL>        Fail build on: critical, high, medium, low
                             (default: high)
    --exit-code              Exit with non-zero code if vulnerabilities found
    -h, --help               Show help message

EXAMPLES:
    # Scan current directory
    confrisk-npm

    # Scan specific project
    confrisk-npm --path /path/to/project

    # Fail CI on high or critical vulnerabilities
    confrisk-npm --fail-on high --exit-code

    # JSON output for parsing
    confrisk-npm --format json > results.json

    # Dev environment (lower risk weights)
    confrisk-npm --asset dev
```

## Git Hooks Integration (Husky)

### Setup

1. **Install husky**

```bash
npm install --save-dev husky
npm pkg set scripts.prepare="husky install"
npm run prepare
```

2. **Add pre-commit hook**

```bash
npx husky add .husky/pre-commit "confrisk-npm --fail-on high --exit-code"
chmod +x .husky/pre-commit
```

3. **Commit will now be blocked if vulnerabilities found!**

```bash
git add .
git commit -m "Update dependencies"
# 🔒 Running confrisk-npm security scan...
# ❌ Security scan failed! Fix vulnerabilities before committing.
```

### Example `.husky/pre-commit`

```bash
#!/usr/bin/env sh
. "$(dirname -- "$0")/_/husky.sh"

# Run confrisk-npm security scan before commit
echo "🔒 Running confrisk-npm security scan..."

confrisk-npm --path . --fail-on high --exit-code

if [ $? -ne 0 ]; then
  echo "❌ Security scan failed! Fix vulnerabilities before committing."
  echo "Run 'npm audit fix' or check the output above."
  exit 1
fi

echo "✅ Security scan passed!"
```

## CI/CD Integration

### GitHub Actions

Create `.github/workflows/security-scan.yml`:

```yaml
name: Security Scan

on:
  push:
    branches: [ main, master ]
  pull_request:
    branches: [ main, master ]

jobs:
  confrisk-npm:
    runs-on: ubuntu-latest

    steps:
      - uses: actions/checkout@v4

      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: '20'

      - name: Install dependencies
        run: npm ci

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Build confrisk-npm
        run: |
          cargo build --release --bin confrisk-npm
          sudo cp target/release/confrisk-npm /usr/local/bin/

      - name: Run security scan
        run: |
          confrisk-npm \
            --path . \
            --asset production \
            --fail-on high \
            --exit-code
```

### GitLab CI

Create `.gitlab-ci.yml`:

```yaml
security-scan:
  stage: test
  image: rust:latest
  before_script:
    - apt-get update && apt-get install -y nodejs npm
    - npm ci
    - cargo build --release --bin confrisk-npm
    - cp target/release/confrisk-npm /usr/local/bin/
  script:
    - confrisk-npm --path . --fail-on high --exit-code
  only:
    - merge_requests
    - master
```

### Jenkins

```groovy
pipeline {
    agent any

    stages {
        stage('Install Dependencies') {
            steps {
                sh 'npm ci'
            }
        }

        stage('Security Scan') {
            steps {
                sh '''
                    cargo build --release --bin confrisk-npm
                    ./target/release/confrisk-npm \
                        --path . \
                        --asset production \
                        --fail-on high \
                        --exit-code
                '''
            }
        }
    }
}
```

### npm Scripts

Add to `package.json`:

```json
{
  "scripts": {
    "security-scan": "confrisk-npm --path . --fail-on high --exit-code",
    "security-scan:dev": "confrisk-npm --path . --asset dev",
    "security-scan:json": "confrisk-npm --format json > security-report.json",
    "pretest": "npm run security-scan",
    "prepublishOnly": "npm run security-scan"
  }
}
```

Then run:

```bash
npm run security-scan
```

## Configuration

### Custom Blocklist

Edit `config/rules/dependencies.json` to add your own blocked packages:

```json
{
  "blocklist": {
    "packages": [
      {
        "name": "old-internal-library",
        "ecosystem": "npm",
        "version_pattern": "^1\\..*",
        "reason": "Company policy: migrate to v2+",
        "severity": "high",
        "alternative": "new-internal-library >= 2.0"
      },
      {
        "name": "banned-package",
        "ecosystem": "npm",
        "reason": "Security vulnerability - no fix available",
        "severity": "critical",
        "alternative": "safe-alternative"
      }
    ]
  }
}
```

### Risk Model Customization

Edit `config/scoring.json`:

```json
{
  "severity": {
    "critical": 15.0,  // Increase critical weight
    "high": 10.0
  },
  "asset_criticality": {
    "production": 2.0,  // Double weight for production
    "dev": 0.3          // Lower for dev
  }
}
```

## Asset Profiles

Choose the right profile for your environment:

| Profile | Use Case | Risk Multiplier |
|---------|----------|-----------------|
| **dev** | Development, local testing | 0.5× |
| **internal** | Internal tools, staging | 0.8× |
| **production** | Production services | 1.1× |
| **crown-jewel** | Critical infrastructure, payment systems | 1.3× |

```bash
# Development scan (less strict)
confrisk-npm --asset dev

# Production scan (strict)
confrisk-npm --asset production --fail-on high --exit-code

# Critical systems (very strict)
confrisk-npm --asset crown-jewel --fail-on medium --exit-code
```

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | No vulnerabilities above threshold |
| 1 | Vulnerabilities found above fail-on threshold |

Use `--exit-code` to fail builds:

```bash
confrisk-npm --fail-on high --exit-code
if [ $? -eq 0 ]; then
  echo "✅ Build passed security scan"
else
  echo "❌ Build failed due to security issues"
fi
```

## JSON Output

Get machine-readable output for processing:

```bash
confrisk-npm --format json > report.json
```

Example output:

```json
[
  {
    "id": "NPM-BLOCKED-LODASH",
    "title": "Blocked package: lodash",
    "description": "Prototype pollution vulnerabilities < 4.17.21",
    "severity": "high",
    "risk": 8.25,
    "priority": 4.13,
    "risk_band": "high",
    "evidence": "Found lodash version ^4.17.20 in dependencies",
    "remediation": "Replace 'lodash' with 'lodash >= 4.17.21'",
    "passed": false,
    "confidence": 0.99,
    "effort": 2.0
  }
]
```

Parse with `jq`:

```bash
# Count critical issues
confrisk-npm --format json | jq '[.[] | select(.risk_band == "critical")] | length'

# Get all blocked packages
confrisk-npm --format json | jq '.[] | select(.id | startswith("NPM-BLOCKED"))'

# Export to CSV
confrisk-npm --format json | jq -r '.[] | [.id, .severity, .risk, .title] | @csv'
```

## Pre-commit Hook Examples

### Basic Hook

```bash
#!/bin/sh
confrisk-npm --fail-on high --exit-code
```

### With Bypass Option

```bash
#!/bin/sh
# Allow bypass with: git commit --no-verify
if [ "$HUSKY_SKIP_SECURITY" = "1" ]; then
  echo "⚠️  Skipping security scan (HUSKY_SKIP_SECURITY=1)"
  exit 0
fi

confrisk-npm --fail-on high --exit-code
```

### Only Scan Changed Files

```bash
#!/bin/sh
# Only scan if package.json or package-lock.json changed
if git diff --cached --name-only | grep -q "package.*\.json"; then
  echo "📦 package.json changed, running security scan..."
  confrisk-npm --fail-on high --exit-code
else
  echo "✅ No package changes, skipping security scan"
fi
```

## Comparison with npm audit

| Feature | npm audit | confrisk-npm |
|---------|-----------|--------------|
| **CVE Detection** | ✅ Yes | ✅ Yes (via npm audit) |
| **Custom Blocklist** | ❌ No | ✅ Yes |
| **Risk Scoring** | Severity only | Contextual (asset + exposure) |
| **Priority Calculation** | ❌ No | ✅ Yes (risk ÷ effort) |
| **Asset Profiles** | ❌ No | ✅ dev, internal, prod, crown-jewel |
| **CI/CD Integration** | Basic | Advanced (exit codes, JSON) |
| **Policy Enforcement** | ❌ No | ✅ Company-wide rules |

## Troubleshooting

### "npm audit not available"

```bash
# Make sure npm is installed
npm --version

# Run manually
npm audit
```

### "Config not found"

```bash
# Specify config path
confrisk-npm --config /path/to/config

# Or copy config directory to project
cp -r confrisk/config ./
```

### "Binary not found in CI"

```bash
# Build and install in CI
cargo build --release --bin confrisk-npm
sudo cp target/release/confrisk-npm /usr/local/bin/
```

## Examples

### Example 1: Block Old lodash

`config/rules/dependencies.json`:

```json
{
  "name": "lodash",
  "ecosystem": "npm",
  "version_pattern": "^4.17.[0-9]$",
  "reason": "Prototype pollution < 4.17.21",
  "severity": "high",
  "alternative": "lodash >= 4.17.21"
}
```

### Example 2: Prevent Deprecated Packages

```json
{
  "name": "request",
  "ecosystem": "npm",
  "reason": "Deprecated, unmaintained since 2020",
  "severity": "medium",
  "alternative": "axios, node-fetch, got"
}
```

### Example 3: Company Internal Policy

```json
{
  "name": "company-old-sdk",
  "ecosystem": "npm",
  "version_pattern": "^1\\..*",
  "reason": "Company policy: all projects must use SDK v2+",
  "severity": "high",
  "alternative": "company-sdk >= 2.0.0"
}
```

## Demo Project

A complete example with husky integration is available:

```bash
cd examples/npm-project-demo

# Install dependencies
npm install

# Run scan
confrisk-npm

# Try to commit (will trigger pre-commit hook)
git add .
git commit -m "Test commit"
# 🔒 Running confrisk-npm security scan...
```

## Roadmap

- [ ] Yarn and pnpm support
- [ ] Python pip scanning (confrisk-pip)
- [ ] Ruby gems scanning (confrisk-gem)
- [ ] Java Maven/Gradle scanning
- [ ] HTML report output
- [ ] SARIF format for GitHub Security tab
- [ ] Auto-fix suggestions
- [ ] Dependency graph visualization

## FAQ

**Q: Does this replace npm audit?**
A: No, it complements it. confrisk-npm uses npm audit for CVE detection but adds custom blocklists and risk scoring.

**Q: Can I use this in a monorepo?**
A: Yes! Run `confrisk-npm --path ./packages/my-package` for each package.

**Q: How do I bypass the pre-commit hook?**
A: Use `git commit --no-verify` or set `HUSKY_SKIP_SECURITY=1`.

**Q: Does it work offline?**
A: Blocklist checks work offline. npm audit requires internet.

**Q: Can I customize the risk model?**
A: Yes! Edit `config/scoring.json` to adjust severity weights, asset multipliers, etc.

---

**Version:** 0.2.0
**Last Updated:** May 23, 2026
**Documentation:** See also `CONFIG_SYSTEM.md` for full configuration reference
