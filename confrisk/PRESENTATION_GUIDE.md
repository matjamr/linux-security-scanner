# confrisk — Project Presentation & Demo Guide

## 📋 Table of Contents

1. [Presentation Overview](#presentation-overview)
2. [Setup Before Presentation](#setup-before-presentation)
3. [Presentation Outline](#presentation-outline)
4. [Demo 1: Basic System Scan](#demo-1-basic-system-scan)
5. [Demo 2: Risk-Based Prioritization](#demo-2-risk-based-prioritization)
6. [Demo 3: Config-Driven Architecture](#demo-3-config-driven-architecture)
7. [Demo 4: NPM Scanner & Git Hooks](#demo-4-npm-scanner--git-hooks)
8. [Demo 5: CI/CD Integration](#demo-5-cicd-integration)
9. [Talking Points](#talking-points)
10. [Q&A Preparation](#qa-preparation)

---

## Presentation Overview

### Total Duration: 15-20 minutes

**Breakdown:**
- Introduction (2 min)
- Problem Statement (3 min)
- Solution Overview (3 min)
- Live Demos (8-10 min)
- Architecture & Config System (2 min)
- Q&A (5 min)

### Key Message

**confrisk** is a generic, config-driven security assessment framework that provides **contextual risk scoring** and **effort-based prioritization**, not just vulnerability listing.

---

## Setup Before Presentation

### 1. Build Everything

```bash
cd /path/to/linux-security-scanner/confrisk

# Build main binary
cargo build --release

# Build npm scanner
cargo build --release --bin confrisk-npm

# Verify builds
./target/release/confrisk --help
./target/release/confrisk-npm --help
```

### 2. Prepare Docker Demo

```bash
# Navigate to demo directory
cd demo

# Build Docker images (do this BEFORE presentation!)
docker compose build

# Pre-generate reports
docker compose run --rm confrisk-demo
docker compose run --rm confrisk-demo-dev
docker compose run --rm confrisk-demo-production

# Verify reports exist
ls -lh out/
# Should see: report.html, dev.html, production.html
```

### 3. Prepare Terminal Windows

**Open 4 terminal windows/tabs:**

**Terminal 1:** Main demo directory
```bash
cd /path/to/linux-security-scanner/confrisk
```

**Terminal 2:** Docker demo
```bash
cd /path/to/linux-security-scanner/confrisk/demo
```

**Terminal 3:** NPM demo
```bash
cd /path/to/linux-security-scanner/confrisk/examples/npm-project-demo
```

**Terminal 4:** Config exploration
```bash
cd /path/to/linux-security-scanner/confrisk/config
```

### 4. Prepare Browser Tabs

Open these in your browser:

1. `file:///path/to/confrisk/demo/out/report.html` (crown-jewel scan)
2. `file:///path/to/confrisk/demo/out/dev.html` (dev scan)
3. `file:///path/to/confrisk/demo/out/production.html` (production scan)
4. GitHub Actions example (if you want to show it)

### 5. Clean Console Output

```bash
# Clear all terminal histories
clear

# Make terminals readable
# - Increase font size
# - Use high-contrast theme
# - Full screen or large windows
```

### 6. Backup Slides/Notes

Have these files open for reference:
- `README.md` — Project overview
- `DEMO_RESULTS.md` — Demo results
- `CONFIG_SYSTEM.md` — Configuration reference
- `CONFRISK_NPM.md` — NPM scanner guide
- This file — Presentation guide

---

## Presentation Outline

### Slide 1: Title (30 seconds)

```
┌─────────────────────────────────────────────┐
│                                             │
│            confrisk                         │
│                                             │
│   Linux Security Assessment Framework       │
│                                             │
│   Risk-Based • Prioritized • Configurable   │
│                                             │
└─────────────────────────────────────────────┘
```

**Say:**
> "Today I'm presenting confrisk — a security assessment framework that goes beyond just finding vulnerabilities. It helps you understand risk and prioritize fixes based on your environment."

---

### Slide 2: The Problem (2 minutes)

**Show these pain points:**

```
❌ Traditional Security Scanners:

1. List hundreds of findings
2. All marked "CRITICAL" or "HIGH"
3. No prioritization
4. No context about YOUR environment
5. Dev and Production treated the same

Result:
→ Alert fatigue
→ Don't know where to start
→ High-effort, low-risk fixes first
```

**Say:**
> "Current security tools give you a list of issues but don't help you prioritize. A critical vulnerability in a development environment might not be as urgent as a medium vulnerability in a production payment system. That's where confrisk comes in."

---

### Slide 3: The Solution (2 minutes)

**Show the formula:**

```
┌─────────────────────────────────────────────┐
│  Risk-Based Scoring Model                   │
│                                             │
│  risk = severity × asset × exposure ×       │
│         confidence                          │
│                                             │
│  priority = risk ÷ effort                   │
│                                             │
│  Result: Fix high-risk, low-effort first!   │
└─────────────────────────────────────────────┘
```

**Key Features:**
- ✅ Contextual risk scoring
- ✅ Effort-based prioritization
- ✅ Explainable AI/ML approach
- ✅ Config-driven (no code changes)
- ✅ Plugin architecture

**Say:**
> "confrisk calculates real risk by considering your environment. The same vulnerability has different risk scores in dev vs production. Then it prioritizes by effort — surface the quick wins first."

---

## Demo 1: Basic System Scan

**Time:** 2 minutes
**Terminal:** Terminal 2 (Docker demo)

### Step 1: Show the Vulnerable Container

```bash
# Show Dockerfile
cat Dockerfile.vulnerable
```

**Point out:**
- SSH root login enabled
- Password authentication enabled
- /etc/shadow readable by all (CRITICAL!)
- World-writable files in /etc

**Say:**
> "This is an intentionally vulnerable container with multiple security misconfigurations. Let's scan it."

### Step 2: Run the Scan

```bash
# Run scan
docker compose run --rm confrisk-demo
```

**Expected output:**
```
confrisk v0.1 — skan zakończony
host: 908a816e8097 | profil: crown-jewel
findingi: 8 (critical: 2, high: 3, medium: 0, low: 1, passed: 2)
skumulowane ryzyko: 47.0
raport: /out/report.html
```

**Say:**
> "In seconds, we scanned the system and found 8 issues: 2 critical, 3 high. Total risk score: 47.0. Let's look at the report."

### Step 3: Open HTML Report

```bash
# Open in browser (or click the pre-opened tab)
open out/report.html
```

**Point out in the report:**
1. **Posture Banner** — "ZAGROŻONY" (THREATENED)
2. **Statistics** — Visual breakdown of findings
3. **Risk Model Box** — Shows the formula
4. **Findings List** — Sorted by priority

**Click on a finding to expand:**
- Show the description
- Show the evidence
- **Show the score breakdown** — This is key!
  - Example: `8.0 (sev:high) × 1.30 (asset:crown-jewel) × 1.25 (expo:internet-facing) × 0.95 (conf) = 12.35`
- Show the remediation steps

**Say:**
> "Notice the score breakdown — this is explainable AI. You can see exactly why this finding has a risk score of 12.35. It's not a black box."

---

## Demo 2: Risk-Based Prioritization

**Time:** 3 minutes
**Terminal:** Terminal 2 (Docker demo)
**Browser:** All 3 report tabs

### The Key Demo: Same Container, Different Risk

**Say:**
> "Now here's where it gets interesting. Let's scan the SAME container with different asset profiles."

### Step 1: Show All Three Scans

**Already done in setup, just show the summary:**

```bash
# Show the scan results
cat << 'EOF'
Same Container, Different Risk:

Dev Profile (×0.5):
  Critical: 0, High: 0, Medium: 1, Low: 4
  Cumulative Risk: 18.1

Production Profile (×1.1):
  Critical: 1, High: 2, Medium: 2, Low: 1
  Cumulative Risk: 39.8

Crown-Jewel Profile (×1.3):
  Critical: 2, High: 3, Medium: 0, Low: 1
  Cumulative Risk: 47.0
EOF
```

### Step 2: Compare in Browser

**Open all 3 tabs side-by-side** (or switch between them):

1. **dev.html** — Green posture, mostly LOW
2. **production.html** — Yellow/orange posture
3. **report.html** (crown-jewel) — RED posture, CRITICAL warnings

**Point out the SAME finding (e.g., SSH root login):**

| Profile | Risk Band | Priority | Action |
|---------|-----------|----------|--------|
| Dev | LOW | 2.1 | Fix when convenient |
| Production | HIGH | 6.8 | Fix soon |
| Crown-Jewel | CRITICAL | 8.9 | Fix NOW! |

**Say:**
> "Look at this — the SAME SSH root login vulnerability. In dev it's low priority (2.1). In production it's high (6.8). In a crown-jewel asset it's critical (8.9). Same vulnerability, different business impact. This is contextual risk assessment."

### Step 3: Show Priority Sorting

**In the crown-jewel report:**
- Scroll through the findings list
- **Point out they're sorted by priority, not severity**
- Show a high-risk, low-effort fix at the top
- Show a medium-severity, high-effort fix lower down

**Say:**
> "Notice the findings are sorted by priority — risk divided by effort. This finding has high risk but takes 1 minute to fix, so it's at the top. This other one has medium risk but requires architectural changes, so it's lower priority. This helps you focus on quick wins."

---

## Demo 3: Config-Driven Architecture

**Time:** 2 minutes
**Terminal:** Terminal 4 (Config directory)

### Step 1: Show Config Structure

```bash
# Show config tree
tree -L 2 config/
# Or if tree not available:
find config -type f | head -15
```

**Expected output:**
```
config/
├── categories.json
├── scoring.json
├── checks/
│   ├── ssh-root-login.json
│   └── shadow-permissions.json
├── plugins/
│   ├── trivy.json
│   ├── lynis.json
│   ├── gitleaks.json
│   └── osv-scanner.json
└── rules/
    ├── dependencies.json
    └── ports.json
```

**Say:**
> "Everything is configurable via JSON files. No code changes needed."

### Step 2: Show a Check Definition

```bash
# Show SSH check
cat config/checks/ssh-root-login.json | jq '.'
```

**Point out:**
- **id:** Unique identifier
- **category:** PRIVILEGES (one of 12 categories)
- **severity, exposure, confidence, effort:** Risk factors
- **detection:** How to check (config_directive)
- **remediation:** How to fix

**Say:**
> "This is a check definition. To add a new check, you just create a JSON file like this. No Rust code needed."

### Step 3: Show Dependencies Blocklist

```bash
# Show blocked packages
cat config/rules/dependencies.json | jq '.blocklist.packages[:3]'
```

**Show examples:**
- event-stream (supply chain attack)
- log4j < 2.17.1 (Log4Shell)
- lodash < 4.17.21 (prototype pollution)

**Say:**
> "You can define company-wide policies. For example, block all versions of log4j before 2.17.1 due to Log4Shell. Or ban deprecated packages like event-stream which had a supply chain attack."

### Step 4: Show Configurable Scoring

```bash
# Show risk weights
cat config/scoring.json | jq '.severity, .asset_criticality'
```

**Say:**
> "Even the risk model is configurable. Want to make critical vulnerabilities count 20x instead of 10x? Just edit this JSON file. Want dev environments to have even lower weight? Change the multiplier. No recompilation needed."

---

## Demo 4: NPM Scanner & Git Hooks

**Time:** 3 minutes
**Terminal:** Terminal 3 (NPM demo)

### Step 1: Show the NPM Scanner

```bash
# Go to npm demo
cd examples/npm-project-demo

# Show package.json
cat package.json | jq '.dependencies'
```

**Point out:**
- lodash: "^4.17.20" (vulnerable version)

**Say:**
> "Now let me show you confrisk-npm — a dedicated scanner for npm projects."

### Step 2: Run NPM Scan

```bash
# Run scan
../../target/release/confrisk-npm --path .
```

**Expected output:**
```
┌────────────────────────────────────────────────────────────┐
│  confrisk-npm — NPM Security Scan                          │
└────────────────────────────────────────────────────────────┘

Project: .
Checks: X

🟠 [NPM-BLOCKED-LODASH] Blocked package: lodash (priority: 4.1)
   Prototype pollution vulnerabilities < 4.17.21
   Evidence: Found lodash version ^4.17.20 in dependencies
   Fix: Replace 'lodash' with 'lodash >= 4.17.21'
```

**Say:**
> "It found the vulnerable lodash version. But here's where it gets interesting — this integrates with git hooks."

### Step 3: Show Git Hook Integration

```bash
# Show the pre-commit hook
cat .husky/pre-commit
```

**Point out:**
```bash
confrisk-npm --path . --fail-on high --exit-code

if [ $? -ne 0 ]; then
  echo "❌ Security scan failed!"
  exit 1
fi
```

**Say:**
> "This is a Husky pre-commit hook. Before allowing a commit, it runs confrisk-npm. If vulnerabilities are found, the commit is blocked."

### Step 4: Demo Blocked Commit (Optional, if time)

```bash
# Make a dummy change
echo "// test" >> index.js

# Try to commit
git add .
git commit -m "Test commit with vulnerabilities"
```

**Expected output:**
```
🔒 Running confrisk-npm security scan...
🟠 [NPM-BLOCKED-LODASH] Blocked package: lodash
❌ Security scan failed! Fix vulnerabilities before committing.
```

**Say:**
> "The commit is blocked! The developer has to fix the vulnerability before they can commit. This shifts security left — catching issues before they enter the codebase."

### Step 5: Show JSON Output

```bash
# JSON output for CI/CD
../../target/release/confrisk-npm --path . --format json | jq '.[0]'
```

**Point out:**
- Machine-readable format
- Can be parsed in CI/CD
- Includes risk, priority, remediation

**Say:**
> "The JSON output can be parsed by CI/CD tools, integrated into dashboards, or used to fail builds automatically."

---

## Demo 5: CI/CD Integration

**Time:** 2 minutes
**Terminal:** Terminal 3 (NPM demo)

### Step 1: Show GitHub Actions Workflow

```bash
# Show workflow
cat .github/workflows/security-scan.yml
```

**Point out key sections:**

```yaml
- name: Run security scan
  run: |
    confrisk-npm \
      --path . \
      --asset production \
      --fail-on high \
      --exit-code
```

**Say:**
> "This GitHub Actions workflow runs on every push and pull request. It fails the build if high or critical vulnerabilities are found."

### Step 2: Show Multiple CI/CD Examples

**Have the file open:**
```bash
# Quick show of other examples
echo "Also provided:"
echo "  - GitLab CI (.gitlab-ci.yml example)"
echo "  - Jenkins Pipeline (Jenkinsfile example)"
echo "  - npm scripts integration"
```

**Say:**
> "We provide ready-to-use templates for GitHub Actions, GitLab CI, and Jenkins. Just copy and paste into your project."

---

## Talking Points

### Why This Project Matters

**1. Contextual Risk Assessment**
> "Traditional scanners treat all environments the same. confrisk understands that a vulnerability in dev is different from production."

**2. Prioritization, Not Just Detection**
> "Security teams get lists of 500 vulnerabilities. confrisk tells you which 5 to fix first based on risk and effort."

**3. Explainability**
> "Every score includes a breakdown. You can audit the decision-making process. No black box AI."

**4. Policy as Code**
> "Security policies are version-controlled JSON files. Block packages, define risk weights, all without code changes."

**5. Developer Experience**
> "Integrates into existing workflows — git hooks, CI/CD, npm scripts. Developers get fast feedback."

### Technical Achievements

**1. Hybrid Risk Model**
> "Combines 5 factors: severity, asset criticality, exposure, confidence, and effort. Novel approach in open-source scanners."

**2. Zero Dependencies**
> "Pure Rust with stdlib only. Single static binary. No npm installs, no Python packages, no containers (except for demos)."

**3. Plugin Architecture**
> "Integrates Trivy, Lynis, Gitleaks, OSV-Scanner without code. Just JSON config files."

**4. Multi-Platform**
> "Works on Linux, macOS, containers. Scans npm, can be extended to pip, gem, Maven."

**5. Production-Ready**
> "Exit codes for CI/CD, JSON output for automation, git hooks for prevention, detailed HTML reports."

### Academic/Research Contributions

**1. Risk Modeling**
> "Demonstrates multi-factor risk assessment with transparency. Each finding shows exactly how its score was calculated."

**2. Effort-Adjusted Prioritization**
> "Novel 'priority = risk ÷ effort' calculation surfaces quick wins. Not just 'what's dangerous' but 'what's dangerous AND easy to fix'."

**3. Config-Driven Security**
> "Shows security tools can be generic and configurable. One framework, unlimited use cases."

**4. Reproducibility**
> "All scans are reproducible. Same config + same system = same results. Critical for compliance."

---

## Q&A Preparation

### Expected Questions & Answers

**Q: How does this compare to OpenSCAP or Lynis?**

A: "OpenSCAP and Lynis are excellent scanners that detect issues. confrisk is a framework that can integrate them (as plugins) and adds contextual risk scoring and prioritization on top. Think of it as an orchestration layer that makes sense of findings from multiple tools."

**Q: Does it work on Windows?**

A: "Currently it's focused on Linux systems, but the config-driven architecture makes it easy to extend. The npm scanner works on any platform. Adding Windows checks is just a matter of creating JSON config files for Windows-specific checks."

**Q: How do you handle false positives?**

A: "Each finding has a confidence score (0.0-1.0) that factors into the risk calculation. Low confidence findings get lower risk scores. You can also disable specific checks in the config."

**Q: Can this replace our existing security tools?**

A: "It's designed to complement, not replace. Use confrisk to aggregate findings from your existing tools (Trivy, Lynis, etc.) and add risk-based prioritization. The plugin system makes this easy."

**Q: What about compliance (PCI-DSS, NIST, etc.)?**

A: "Each check can be tagged with compliance mappings. We have a COMPLIANCE category. You can tag checks with 'CIS Benchmark 5.2.10' or 'NIST 800-53 AC-6' and generate compliance reports."

**Q: How do you keep the vulnerability database updated?**

A: "For dependencies, we integrate with npm audit (always up-to-date) and can integrate OSV-Scanner (Google's database). For system checks, you update the JSON config files — pull from a central repository or use automated config management."

**Q: What's the performance impact?**

A: "Minimal. System scans take seconds (8 checks run in < 1 second). npm audit depends on npm's speed. The overhead is just the risk calculation and report generation."

**Q: Can we customize the HTML report?**

A: "Yes! The report generator is in `src/report.rs`. You can modify the template or create your own report format. We also have JSON output if you want to build custom dashboards."

**Q: Is this production-ready?**

A: "The core framework and npm scanner are production-ready. They're tested, have error handling, and follow best practices. The plugin system is alpha — we have configs but the runner needs more work."

**Q: How do you handle updates to the tool itself?**

A: "Binary releases (planned) or build from source. Config files are separate, so updates don't break your custom rules. Version compatibility is tracked in config files."

---

## Presentation Checklist

### Before You Start

- [ ] All terminals open and positioned
- [ ] All browser tabs loaded
- [ ] Docker images built
- [ ] Reports pre-generated
- [ ] Font sizes increased in terminals
- [ ] Internet connection verified (if showing GitHub)
- [ ] Timer ready (15-20 min)
- [ ] Water nearby
- [ ] Slides/notes accessible

### During Presentation

- [ ] Speak slowly and clearly
- [ ] Point to what you're showing
- [ ] Pause after demos for questions
- [ ] Show enthusiasm!
- [ ] Make eye contact with audience
- [ ] Use the word "**contextual**" often
- [ ] Emphasize "**explainable**" scoring
- [ ] Highlight "**no code changes**" for configs

### After Each Demo

- [ ] Ask "Any questions on this part?"
- [ ] Summarize what was shown
- [ ] Connect to the next demo

### Key Phrases to Use

✅ "Contextual risk assessment"
✅ "Explainable AI approach"
✅ "Shift security left"
✅ "Policy as code"
✅ "Quick wins prioritization"
✅ "Same vulnerability, different risk"
✅ "No code changes needed"

### Things to Avoid

❌ Don't say "just a class project" — this is research
❌ Don't apologize for missing features — focus on what works
❌ Don't rush through the risk calculation — it's the key innovation
❌ Don't skip the score breakdown in HTML reports — it's unique!

---

## Backup Demos (If Extra Time)

### Backup Demo 1: Create a Custom Check

```bash
# Create new check
cat > config/checks/custom-demo.json << 'EOF'
{
  "id": "DEMO-001",
  "name": "Custom check demo",
  "category": "COMPLIANCE",
  "severity": "medium",
  "exposure": "local",
  "confidence": 0.9,
  "effort": "trivial",
  "enabled": true,
  "detection": {
    "type": "file_exists",
    "file": "/etc/custom-security-flag",
    "should_exist": true
  },
  "remediation": {
    "summary": "Create security flag file",
    "steps": ["touch /etc/custom-security-flag"]
  }
}
EOF

# Show it was added
ls -l config/checks/

# Explain: "No recompilation needed, it's automatically loaded!"
```

### Backup Demo 2: Block a Package

```bash
# Add to blocklist
cat config/rules/dependencies.json | \
  jq '.blocklist.packages += [{
    "name": "custom-banned-lib",
    "ecosystem": "npm",
    "reason": "Demo: company policy violation",
    "severity": "high",
    "alternative": "approved-lib"
  }]' > /tmp/deps.json

# Show the change
jq '.blocklist.packages[-1]' /tmp/deps.json
```

### Backup Demo 3: Customize Risk Weights

```bash
# Show current weights
cat config/scoring.json | jq '.severity'

# Explain how to change:
echo "To make critical 2x more important:"
echo "  Change 'critical': 10.0 to 'critical': 20.0"
echo "Then re-run scan — different priorities!"
```

---

## Closing (1 minute)

### Summary Points

**Show this slide:**

```
┌─────────────────────────────────────────────┐
│  confrisk Summary                            │
│                                             │
│  ✅ Contextual risk scoring                 │
│  ✅ Effort-based prioritization             │
│  ✅ Explainable decisions                   │
│  ✅ Config-driven (no code changes)         │
│  ✅ Plugin architecture                     │
│  ✅ Git hooks + CI/CD integration           │
│  ✅ Production-ready npm scanner            │
│                                             │
│  GitHub: [your-repo-url]                    │
│  Docs: README.md, CONFIG_SYSTEM.md          │
└─────────────────────────────────────────────┘
```

**Say:**
> "To summarize: confrisk is a security assessment framework that understands context. Same vulnerability, different risk based on your environment. It prioritizes by effort to help you focus on quick wins. Everything is configurable via JSON — no code changes. And it integrates into your existing workflows with git hooks and CI/CD."

> "All code and documentation are available. Thank you! Questions?"

---

## Post-Presentation

### Share These Files

1. `README.md` — Project overview
2. `CONFIG_SYSTEM.md` — Configuration guide
3. `CONFRISK_NPM.md` — NPM scanner guide
4. `PRESENTATION_GUIDE.md` — This file
5. `demo/out/report.html` — Example report

### Follow-Up Materials

Email attendees:
```
Subject: confrisk — Security Assessment Framework

Thank you for attending the presentation!

Repository: [URL]

Key Documents:
- README.md — Project overview and quick start
- CONFIG_SYSTEM.md — Complete configuration reference
- CONFRISK_NPM.md — NPM scanner guide
- examples/npm-project-demo/ — Working example with git hooks

Try it:
  git clone [URL]
  cd confrisk
  cargo build --release
  cargo run -- --asset dev

Questions? [your email]
```

---

## Troubleshooting During Presentation

### If Docker fails

**Backup plan:**
> "Let me show you the pre-generated reports instead."

Open the HTML files directly.

### If build fails

**Backup plan:**
> "I have pre-built binaries here."

Use the already-built binaries from setup.

### If npm demo fails

**Backup plan:**
> "Let me show you the JSON output I prepared earlier."

Have a JSON file ready:
```bash
cat examples/npm-output-example.json
```

### If you forget a command

**Backup plan:**
> "Let me check my notes quickly."

Have this file open in a tab for quick reference.

### If time runs short

**Skip these in order:**
1. Backup demos
2. Demo 3 (config system) — show files only, don't edit
3. Demo 5 (CI/CD) — just mention it exists

**Never skip:**
- Demo 1 (basic scan)
- Demo 2 (risk prioritization) — THIS IS THE KEY DEMO!

---

## Presentation Success Metrics

### You Crushed It If:

✅ Audience says "Wow" at the risk score differences (Demo 2)
✅ Someone asks "Can I use this in my project?"
✅ Questions about customization/extension
✅ People take photos of the risk formula
✅ Someone asks for the GitHub link

### Red Flags:

❌ People look confused during risk calculation
❌ No questions at all
❌ Questions about "why not just use npm audit"
❌ Audience on phones (losing attention)

### Recovery:

If losing audience:
> "Let me show you something cool — watch this..."

Then jump to Demo 2 (risk prioritization) — the most impressive part.

---

**Good luck with your presentation! You've got this! 🚀**

**Remember:** The key innovation is **contextual risk scoring**. Emphasize that the same vulnerability has different risks in different environments. That's what makes this unique.
