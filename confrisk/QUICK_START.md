# confrisk — Quick Start Guide

**Get started in 5 minutes!**

---

## 📚 Documentation Navigator

| Document | Purpose | When to Use |
|----------|---------|-------------|
| **[README.md](README.md)** | Project overview | First-time visitors |
| **[QUICK_START.md](QUICK_START.md)** | This file! | Getting started fast |
| **[PRESENTATION_GUIDE.md](PRESENTATION_GUIDE.md)** | Step-by-step demo guide | Presenting the project |
| **[TECHNICAL_REPORT.md](TECHNICAL_REPORT.md)** | Technical analysis + diagrams | Understanding architecture |
| **[CONFRISK_NPM.md](CONFRISK_NPM.md)** | NPM scanner docs | Using npm scanner |
| **[PUBLISHING.md](PUBLISHING.md)** | Distribution guide | Publishing to repositories |
| **[CONFIG_SYSTEM.md](CONFIG_SYSTEM.md)** | Configuration reference | Customizing checks |

---

## 🚀 Quick Start

### 1. Build the Project

```bash
# Clone the repository
cd confrisk

# Build both binaries
cargo build --release

# Binaries created:
# - target/release/confrisk       (system scanner)
# - target/release/confrisk-npm   (npm dependency scanner)
```

### 2. Run Your First Scan

#### System Scan

```bash
./target/release/confrisk --asset production --out report.html
open report.html
```

#### NPM Project Scan

```bash
./target/release/confrisk-npm --path examples/npm-project-demo --config config
```

### 3. Try the Demo

```bash
# Build demo container
cd demo
docker-compose up -d

# Scan the vulnerable container
../target/release/confrisk --asset dev --out dev-report.html
../target/release/confrisk --asset production --out prod-report.html

# Compare the reports (same vulnerabilities, different risk scores!)
open dev-report.html
open prod-report.html
```

---

## 📊 For Presentations

### Quick Demo Script (2 minutes)

```bash
# 1. Show contextual risk scoring
./target/release/confrisk --asset dev --out dev.html
./target/release/confrisk --asset crown-jewel --out crown.html

# 2. Show NPM scanning
cd examples/npm-project-demo
npm install
../../target/release/confrisk-npm --config ../../config

# 3. Show git hooks (blocks commits)
echo "// test" >> index.js
git add .
git commit -m "Test"
# ❌ Blocked!
```

**Full presentation guide:** [PRESENTATION_GUIDE.md](PRESENTATION_GUIDE.md)

---

## 🔧 Installation

### Option 1: Build from Source

```bash
cargo build --release
sudo cp target/release/confrisk /usr/local/bin/
sudo cp target/release/confrisk-npm /usr/local/bin/
```

### Option 2: Use Install Script

```bash
chmod +x install.sh
./install.sh
```

### Option 3: Build .deb Package

```bash
chmod +x scripts/build-deb.sh
./scripts/build-deb.sh

# Install
sudo dpkg -i releases/confrisk_0.2.0_amd64.deb
sudo dpkg -i releases/confrisk-npm_0.2.0_amd64.deb
```

---

## 🎯 Common Use Cases

### 1. Scan Linux System

```bash
confrisk --asset production --out security-report.html
```

### 2. Scan NPM Project

```bash
confrisk-npm --path /path/to/project --fail-on high --exit-code
```

### 3. Add to Git Hooks (Husky)

```bash
# In your npm project:
npm install --save-dev husky
npx husky add .husky/pre-commit "confrisk-npm --fail-on high --exit-code"
```

### 4. Add to GitHub Actions

```yaml
# .github/workflows/security.yml
- name: Security Scan
  run: |
    cargo build --release --bin confrisk-npm
    ./target/release/confrisk-npm --fail-on high --exit-code
```

### 5. Customize Security Checks

```bash
# Edit config files (no code changes needed!)
vim config/rules/dependencies.json  # Add blocked packages
vim config/checks/ssh-*.json        # Modify SSH checks
vim config/scoring.json             # Adjust risk weights
```

---

## 📖 Key Concepts

### Contextual Risk Scoring

Same vulnerability = different risk scores based on context:

```bash
# Dev environment (relaxed)
confrisk --asset dev
# Risk: SSH root login = 2.5 (LOW)

# Production (strict)
confrisk --asset production
# Risk: SSH root login = 10.9 (CRITICAL)
```

### Risk Formula

```
Risk = Severity × Asset_Criticality × Exposure × Confidence
Priority = Risk ÷ Effort
```

### Asset Profiles

| Profile | Multiplier | Use Case |
|---------|-----------|----------|
| **dev** | 0.5× | Development environments |
| **internal** | 0.8× | Internal tools, staging |
| **production** | 1.1× | Production services |
| **crown-jewel** | 1.3× | Critical infrastructure |

---

## 🔬 Examples

### Example 1: Find High-Risk Issues

```bash
confrisk --asset production --out report.html

# Open report.html and look for:
# - 🔴 Critical (9.0+)
# - 🟠 High (6.0-9.0)
```

### Example 2: Block Vulnerable NPM Packages

```bash
# Add to config/rules/dependencies.json:
{
  "name": "old-package",
  "ecosystem": "npm",
  "reason": "Company policy: migrate to new-package",
  "severity": "high"
}

# Scan will now fail if old-package is found
confrisk-npm --fail-on high --exit-code
```

### Example 3: JSON Output for Automation

```bash
# Get JSON output
confrisk-npm --format json > findings.json

# Parse with jq
jq '.[] | select(.risk_band == "critical")' findings.json
jq '[.[] | select(.risk >= 6.0)] | length' findings.json
```

---

## 🎨 Customization

### Add New Security Check

Create `config/checks/my-check.json`:

```json
{
  "id": "MY-CUSTOM-CHECK",
  "title": "Check custom setting",
  "category": "KERNEL",
  "severity": "high",
  "detection": {
    "type": "file_exists",
    "file": "/etc/my-config",
    "should_exist": true
  },
  "remediation": "Install my-config package"
}
```

No code changes needed! The scanner automatically picks up new checks.

### Adjust Risk Weights

Edit `config/scoring.json`:

```json
{
  "severity": {
    "critical": 20.0,  // Increase critical weight
    "high": 12.0
  },
  "asset_criticality": {
    "production": 2.0   // Double production risk
  }
}
```

---

## 🐛 Troubleshooting

### Build Errors

```bash
# Update Rust
rustup update

# Clean and rebuild
cargo clean
cargo build --release
```

### Config Not Found

```bash
# Specify config path explicitly
confrisk-npm --config /path/to/config
```

### Permission Denied

```bash
# Install with sudo
sudo cp target/release/confrisk* /usr/local/bin/
```

---

## 📊 Project Statistics

| Metric | Value |
|--------|-------|
| **Lines of Code (Rust)** | ~2,500 |
| **Documentation** | ~10,000 lines |
| **Mermaid Diagrams** | 25+ |
| **Security Categories** | 12 |
| **Example Checks** | 8+ |
| **Scanners** | 2 (system + npm) |

---

## 🎓 Learning Path

### For Developers

1. Read [README.md](README.md) — Understand what confrisk does
2. Build and run your first scan (see above)
3. Read [CONFRISK_NPM.md](CONFRISK_NPM.md) — Learn npm scanning
4. Add git hooks to your project
5. Read [CONFIG_SYSTEM.md](CONFIG_SYSTEM.md) — Customize checks

### For Security Teams

1. Read [TECHNICAL_REPORT.md](TECHNICAL_REPORT.md) — Architecture overview
2. Review risk scoring model
3. Customize blocklists in `config/rules/`
4. Deploy to CI/CD pipelines
5. Create company-wide policies

### For Presenters

1. Read [PRESENTATION_GUIDE.md](PRESENTATION_GUIDE.md)
2. Practice 5 live demos
3. Review Q&A section
4. Set up demo environment
5. Present!

---

## 🚢 Publishing

Want to distribute confrisk via `apt-get install`?

See **[PUBLISHING.md](PUBLISHING.md)** for:
- Creating .deb packages
- Publishing to Ubuntu PPA
- Snapcraft distribution
- GitHub Releases
- cargo publish

---

## 🗺️ Roadmap

### v0.3 (Q3 2026)
- Python pip scanner
- Ruby gem scanner
- Java Maven scanner

### v0.4 (Q4 2026)
- Auto-fix engine
- Dependency graph visualization
- SARIF output format

### v1.0 (Q2 2027)
- Web dashboard
- ML-powered risk prediction
- Enterprise features

Full roadmap with diagrams: [TECHNICAL_REPORT.md](TECHNICAL_REPORT.md)

---

## 💡 Key Innovations

### 1. Contextual Risk Assessment
**Problem:** Traditional scanners treat all environments equally
**Solution:** Risk scores adjust based on asset criticality

### 2. Config-Driven Architecture
**Problem:** Adding security checks requires code changes
**Solution:** JSON-based configuration, zero code changes

### 3. Explainable Scoring
**Problem:** "High severity" doesn't explain impact
**Solution:** Every score includes detailed calculation breakdown

### 4. Multi-Scanner Approach
**Problem:** Need different tools for system vs dependencies
**Solution:** Unified framework with specialized scanners

---

## 🎯 Success Stories

### Prevents Vulnerable Commits

```bash
$ git commit -m "Update dependencies"
🔒 Running confrisk-npm security scan...
❌ Security scan failed! Fix vulnerabilities before committing.

🟠 [NPM-BLOCKED-LODASH] Blocked package: lodash
   Fix: npm install lodash@latest
```

### Contextual Risk Prioritization

| Environment | SSH Root Login Risk | Action |
|-------------|---------------------|--------|
| Dev | 2.5 (LOW) | Monitor |
| Staging | 6.8 (HIGH) | Schedule fix |
| Production | 10.9 (CRITICAL) | **Fix immediately** |

### Company-Wide Policy Enforcement

```json
// Block old internal SDK
{
  "name": "company-old-sdk",
  "ecosystem": "npm",
  "reason": "Migrate to v2+ per security policy",
  "severity": "high"
}
```

All projects automatically enforce this rule!

---

## 📞 Get Help

- **Documentation:** [README.md](README.md), [CONFRISK_NPM.md](CONFRISK_NPM.md)
- **Issues:** https://github.com/yourusername/confrisk/issues
- **Discussions:** https://github.com/yourusername/confrisk/discussions

---

## 🎉 Next Steps

1. ✅ Build the project
2. ✅ Run your first scan
3. ✅ Try the demo
4. ✅ Add to your workflow
5. ✅ Share with your team!

**Ready to present?** See [PRESENTATION_GUIDE.md](PRESENTATION_GUIDE.md)

**Want technical details?** See [TECHNICAL_REPORT.md](TECHNICAL_REPORT.md)

**Ready to publish?** See [PUBLISHING.md](PUBLISHING.md)

---

**Version:** 0.2.0
**Status:** Production Ready
**License:** MIT
