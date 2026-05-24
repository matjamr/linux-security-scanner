# confrisk-npm Demo Project

This is a demonstration npm project with intentionally vulnerable dependencies for testing **confrisk-npm**.

## What's Included

### Vulnerable Dependencies

This project includes:
- `lodash@4.17.20` — Has prototype pollution vulnerability (fixed in 4.17.21)
- Older versions that will trigger npm audit warnings

### Git Hooks (Husky)

Pre-commit hook that runs `confrisk-npm` before allowing commits:
- `.husky/pre-commit` — Blocks commits if high/critical vulnerabilities found

### CI/CD Integration

GitHub Actions workflow that:
- Runs `confrisk-npm` on every push/PR
- Fails build if vulnerabilities found
- Uploads security report as artifact
- Comments on PRs with results

## Quick Start

### 1. Install Dependencies

```bash
npm install
```

### 2. Run Security Scan

```bash
# Using npm script
npm run security-scan

# Or directly
confrisk-npm --path . --fail-on high --exit-code
```

### 3. Expected Output

```
┌─ Summary ─────────────────────────────────────────────────┐
│ Critical:   0                                              │
│ High:       1                                              │
│ Medium:     2                                              │
│ Low:        0                                              │
│ Passed:     5                                              │
└───────────────────────────────────────────────────────────┘

🟠 [NPM-BLOCKED-LODASH] Blocked package: lodash (priority: 4.1)
   Prototype pollution vulnerabilities < 4.17.21
   Evidence: Found lodash version ^4.17.20 in dependencies
   Fix: Replace 'lodash' with 'lodash >= 4.17.21'
```

### 4. Fix Vulnerabilities

```bash
# Update lodash
npm install lodash@latest

# Or run npm audit fix
npm audit fix

# Re-run scan
npm run security-scan
```

## Husky Integration

### Test Pre-commit Hook

```bash
# Make a change
echo "// comment" >> index.js

# Try to commit
git add .
git commit -m "Test commit"

# Output:
# 🔒 Running confrisk-npm security scan...
# ❌ Security scan failed! Fix vulnerabilities before committing.
```

### Bypass Hook (Not Recommended)

```bash
# Skip hook for emergency commits
git commit --no-verify -m "Emergency fix"

# Or set environment variable
HUSKY_SKIP_SECURITY=1 git commit -m "Skip security scan"
```

## GitHub Actions

The `.github/workflows/security-scan.yml` workflow:

1. **Runs on**:
   - Every push to main/master/develop
   - Every pull request
   - Daily at 2 AM UTC

2. **Actions**:
   - Checks out code
   - Installs Node.js and dependencies
   - Builds confrisk-npm from source
   - Runs security scan
   - Uploads JSON report as artifact
   - Comments on PRs with results
   - Fails build if high/critical vulnerabilities found

## Configuration

### Customize Fail Threshold

Edit `.husky/pre-commit`:

```bash
# Fail on medium or higher
confrisk-npm --fail-on medium --exit-code

# Fail on any vulnerability
confrisk-npm --fail-on low --exit-code
```

### Customize Asset Profile

Edit `package.json`:

```json
{
  "scripts": {
    "security-scan": "confrisk-npm --asset crown-jewel --fail-on high"
  }
}
```

### Add Custom Blocklist

Edit `../../config/rules/dependencies.json` to block additional packages:

```json
{
  "name": "your-package",
  "ecosystem": "npm",
  "reason": "Your reason here",
  "severity": "high",
  "alternative": "alternative-package"
}
```

## Testing Different Scenarios

### Scenario 1: All Vulnerabilities Fixed

```bash
npm install lodash@latest
npm run security-scan
# ✅ No security issues found!
```

### Scenario 2: Development Environment

```bash
# Lower risk weights for dev
confrisk-npm --asset dev
```

### Scenario 3: JSON Output

```bash
npm run security-scan:json
cat security-report.json | jq '.[] | select(.risk_band == "high")'
```

## Files

```
npm-project-demo/
├── .github/
│   └── workflows/
│       └── security-scan.yml    ← GitHub Actions workflow
├── .husky/
│   └── pre-commit               ← Git pre-commit hook
├── package.json                 ← npm dependencies (vulnerable)
├── index.js                     ← Demo application
└── README.md                    ← This file
```

## Usage Examples

### Basic Scan

```bash
confrisk-npm
```

### Fail on Any Vulnerability

```bash
confrisk-npm --fail-on low --exit-code
```

### Scan with Custom Config

```bash
confrisk-npm --config /path/to/custom/config
```

### Different Asset Profiles

```bash
# Development (relaxed)
confrisk-npm --asset dev

# Production (strict)
confrisk-npm --asset production --fail-on high --exit-code

# Critical systems (very strict)
confrisk-npm --asset crown-jewel --fail-on medium --exit-code
```

## Cleanup

### Remove Husky Hooks

```bash
npm uninstall husky
rm -rf .husky
```

### Update All Dependencies

```bash
npm update
npm audit fix
```

## Learn More

- **Main README**: `../../CONFRISK_NPM.md`
- **Configuration**: `../../CONFIG_SYSTEM.md`
- **Config Directory**: `../../config/`

## License

MIT (Demo/Educational purposes)
