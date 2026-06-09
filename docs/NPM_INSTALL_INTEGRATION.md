# confrisk-npm — Integration with npm install

## Overview

Make `npm install` automatically scan for security vulnerabilities and **block installation** if issues are found.

## 🎯 What You Want

```bash
$ npm install axios

# Automatically runs security scan
🔒 Scanning dependencies...
❌ Security scan failed!
Found: lodash 4.17.20 (CRITICAL)

# Installation blocked! ✋
```

---

## 🚀 Quick Setup (5 minutes)

### Option 1: postinstall Hook (Recommended)

**Best for:** Automatic scanning after every `npm install`

#### Step 1: Install confrisk-npm

```bash
# Local installation (recommended)
npm install --save-dev confrisk-npm

# OR global installation
npm install -g confrisk-npm
```

#### Step 2: Add to package.json

```json
{
  "name": "your-project",
  "scripts": {
    "postinstall": "confrisk-npm --fail-on high --exit-code"
  },
  "devDependencies": {
    "confrisk-npm": "^0.2.0"
  }
}
```

#### Step 3: Test It

```bash
# Install a vulnerable package
npm install lodash@4.17.20

# Output:
# > your-project@1.0.0 postinstall
# > confrisk-npm --fail-on high --exit-code
#
# ❌ Security scan failed!
# 🟠 [NPM-BLOCKED-LODASH] Blocked package: lodash 4.17.20
# npm ERR! Exit code: 1
```

**Result:** Installation completes, then postinstall fails and shows error ⚠️

---

### Option 2: Wrapper Script (Blocks BEFORE install)

**Best for:** Prevent installation entirely

#### Step 1: Install confrisk-npm globally

```bash
npm install -g confrisk-npm
# OR build from source
cargo build --release --bin confrisk-npm
sudo cp target/release/confrisk-npm /usr/local/bin/
```

#### Step 2: Use wrapper script

```bash
# Copy wrapper script
cp bin/npm-install-safe.sh ~/npm-install-safe
chmod +x ~/npm-install-safe

# Use it instead of npm install
~/npm-install-safe axios lodash@4.17.20
```

#### Step 3: Create alias (optional)

```bash
# Add to ~/.bashrc or ~/.zshrc
alias npmi='~/npm-install-safe'

# Now use:
npmi axios
```

---

### Option 3: Pre-commit Hook (Prevent commits)

**Best for:** Block commits with vulnerabilities

#### Step 1: Install Husky

```bash
npm install --save-dev husky
npx husky install
```

#### Step 2: Add pre-commit hook

```bash
npx husky add .husky/pre-commit "npm run security-scan"
```

#### Step 3: Add script to package.json

```json
{
  "scripts": {
    "security-scan": "confrisk-npm --fail-on high --exit-code"
  }
}
```

#### Step 4: Test it

```bash
# Try to commit with vulnerabilities
git add .
git commit -m "Update dependencies"

# Output:
# 🔒 Running security scan...
# ❌ Security scan failed!
# husky - pre-commit hook exited with code 1
```

---

### Option 4: npm Scripts Alias

**Best for:** Manual control

#### Add to package.json:

```json
{
  "scripts": {
    "install:safe": "npm install && confrisk-npm --fail-on high --exit-code",
    "add:safe": "npm install $1 && confrisk-npm --fail-on high --exit-code"
  }
}
```

#### Usage:

```bash
npm run install:safe
# Installs, then scans, exits if vulnerabilities found
```

---

## 📋 Complete Example

### Real-World Setup

**File: `package.json`**
```json
{
  "name": "my-secure-app",
  "version": "1.0.0",
  "scripts": {
    "postinstall": "confrisk-npm --config ./security-config --fail-on high --exit-code",
    "precommit": "confrisk-npm --fail-on critical --exit-code",
    "security-scan": "confrisk-npm --format json > security-report.json",
    "security-check": "confrisk-npm --fail-on medium --exit-code"
  },
  "devDependencies": {
    "confrisk-npm": "^0.2.0",
    "husky": "^8.0.0"
  },
  "husky": {
    "hooks": {
      "pre-commit": "npm run precommit"
    }
  }
}
```

### Configuration File

**File: `security-config/rules/dependencies.json`**
```json
{
  "blocked_packages": [
    {
      "name": "lodash",
      "ecosystem": "npm",
      "version_pattern": "^4\\.17\\.[0-9]$",
      "reason": "Security policy: Use lodash >= 4.17.21",
      "severity": "high"
    },
    {
      "name": "old-company-sdk",
      "ecosystem": "npm",
      "reason": "Internal policy: Deprecated, use new-sdk instead",
      "severity": "critical"
    }
  ]
}
```

---

## 🎬 Demo: How It Works

### Scenario 1: Installing Safe Package

```bash
$ npm install axios

> my-app@1.0.0 postinstall
> confrisk-npm --fail-on high --exit-code

┌────────────────────────────────────────────────────────────┐
│  confrisk-npm — NPM Security Scan                          │
└────────────────────────────────────────────────────────────┘

Project: .
Checks: 5

┌─ Summary ─────────────────────────────────────────────────┐
│ Critical:   0                                              │
│ High:       0                                              │
│ Medium:     0                                              │
│ Low:        0                                              │
│ Passed:     5                                              │
└───────────────────────────────────────────────────────────┘

✅ No security issues found!
```

**Result:** ✅ Installation succeeds

---

### Scenario 2: Installing Vulnerable Package

```bash
$ npm install lodash@4.17.20

> my-app@1.0.0 postinstall
> confrisk-npm --fail-on high --exit-code

┌────────────────────────────────────────────────────────────┐
│  confrisk-npm — NPM Security Scan                          │
└────────────────────────────────────────────────────────────┘

Project: .
Checks: 6

🟠 [NPM-BLOCKED-LODASH] Blocked package: lodash
   Package:      lodash@4.17.20
   Reason:       Prototype pollution vulnerabilities < 4.17.21
   Severity:     HIGH
   Fix:          npm install lodash@latest

   Risk:         8.5 (HIGH)
   Priority:     4.25

┌─ Summary ─────────────────────────────────────────────────┐
│ Critical:   0                                              │
│ High:       1                                              │
│ Medium:     0                                              │
│ Low:        0                                              │
│ Passed:     5                                              │
└───────────────────────────────────────────────────────────┘

❌ Security scan failed!

npm ERR! code 1
npm ERR! path /path/to/project
npm ERR! command failed
```

**Result:** ❌ Installation fails, package is installed but postinstall exits with code 1

---

## 🔧 Advanced Configurations

### 1. Different Severity Levels

```json
{
  "scripts": {
    "postinstall": "confrisk-npm --fail-on critical --exit-code",
    "pre-commit": "confrisk-npm --fail-on high --exit-code",
    "pre-push": "confrisk-npm --fail-on medium --exit-code"
  }
}
```

**Behavior:**
- `postinstall`: Only fails on CRITICAL vulnerabilities
- `pre-commit`: Fails on HIGH or CRITICAL
- `pre-push`: Fails on MEDIUM, HIGH, or CRITICAL

### 2. Environment-Specific

```json
{
  "scripts": {
    "postinstall": "node -e \"if (process.env.NODE_ENV === 'production') { require('child_process').execSync('confrisk-npm --fail-on high --exit-code', {stdio: 'inherit'}) }\""
  }
}
```

**Behavior:** Only scans in production environment

### 3. Skip for CI/CD

```json
{
  "scripts": {
    "postinstall": "if [ -z \"$CI\" ]; then confrisk-npm --fail-on high --exit-code; fi"
  }
}
```

**Behavior:** Skips scan in CI/CD (CI environment variable set)

### 4. Custom Config Path

```json
{
  "scripts": {
    "postinstall": "confrisk-npm --config ./company-security-rules --fail-on high --exit-code"
  }
}
```

---

## 🚫 Bypassing Security Checks

### Temporary Bypass (Not Recommended)

```bash
# Skip postinstall scripts
npm install --ignore-scripts

# Or set environment variable
CONFRISK_SKIP=1 npm install
```

### Permanent Bypass (Really Not Recommended)

```json
{
  "scripts": {
    "postinstall": "confrisk-npm --fail-on high --exit-code || true"
  }
}
```

**Warning:** `|| true` always exits with 0, defeating the purpose!

---

## 📦 Publishing confrisk-npm to npm Registry

### Step 1: Prepare package.json

Already created at `confrisk/package.json` ✅

### Step 2: Build binary

```bash
cargo build --release --bin confrisk-npm
```

### Step 3: Test locally

```bash
# Link globally
npm link

# Test in another project
cd /path/to/test/project
npm link @confrisk/npm
confrisk-npm --help
```

### Step 4: Publish to npmjs.com

```bash
# Login to npm
npm login

# Publish (scoped package)
npm publish --access public

# Or unscoped
# Change name to "confrisk-npm" in package.json
npm publish
```

### Step 5: Users can install

```bash
# Global
npm install -g @confrisk/npm

# Local (per project)
npm install --save-dev @confrisk/npm
```

---

## 🎯 Comparison of Methods

| Method | When Scans | Blocks Install | Ease of Setup | Best For |
|--------|-----------|----------------|---------------|----------|
| **postinstall** | After install | ❌ No (warns only) | ⭐⭐⭐ Easy | Most projects |
| **Wrapper script** | Before install | ✅ Yes | ⭐⭐ Medium | Power users |
| **Pre-commit hook** | Before commit | ✅ Yes | ⭐⭐⭐ Easy | Git workflow |
| **CI/CD** | On push | ✅ Yes | ⭐⭐ Medium | Enterprise |

---

## 🏆 Recommended Setup

### For Most Projects:

```bash
# 1. Install
npm install --save-dev confrisk-npm husky

# 2. Setup Husky
npx husky install

# 3. Add hooks
npx husky add .husky/pre-commit "confrisk-npm --fail-on high --exit-code"

# 4. Add to package.json
{
  "scripts": {
    "postinstall": "confrisk-npm --fail-on critical --exit-code",
    "prepare": "husky install"
  }
}
```

**Result:**
- ✅ Scans after every `npm install` (fails on critical)
- ✅ Blocks commits with vulnerabilities (fails on high)
- ✅ Easy for team to use

---

## 🔥 Real-World Example

### Before confrisk-npm:

```bash
$ npm install
# Installs 1000+ packages
# Including 5 with critical vulnerabilities
# ✅ Success! (but with hidden dangers)
```

### After confrisk-npm:

```bash
$ npm install

> my-app@1.0.0 postinstall
> confrisk-npm --fail-on high --exit-code

❌ Security scan failed!

🔴 [NPM-CRITICAL-1] CVE-2021-12345 in package-a
   Fix: npm install package-a@latest

🟠 [NPM-HIGH-1] Prototype pollution in package-b
   Fix: npm install package-b@^2.0.0

npm ERR! code 1
```

**Developer fixes issues:**
```bash
$ npm install package-a@latest package-b@^2.0.0
$ npm install

✅ No security issues found!
```

---

## 🎓 FAQ

### Q: Does postinstall slow down npm install?
**A:** Yes, ~500ms overhead. But it's worth it for security!

### Q: Can I use this with yarn/pnpm?
**A:** Yes! Use `postinstall` script, it works with all package managers.

```json
{
  "scripts": {
    "postinstall": "confrisk-npm --fail-on high --exit-code"
  }
}
```

### Q: What if confrisk-npm isn't installed yet?
**A:** Use conditional check:

```json
{
  "scripts": {
    "postinstall": "command -v confrisk-npm >/dev/null 2>&1 && confrisk-npm --fail-on high --exit-code || true"
  }
}
```

### Q: Can I auto-fix vulnerabilities?
**A:** Not yet, but coming in v0.3! For now:

```bash
# Manual fix
npm audit fix

# Or update specific package
npm install package@latest
```

---

## 📊 Summary

### ✅ You Can Now:

1. **Auto-scan** on every `npm install` (postinstall)
2. **Block installs** with wrapper script
3. **Prevent commits** with git hooks
4. **Enforce policies** via custom config
5. **Publish to npm** for easy distribution

### 🎯 Recommended Flow:

```
Developer runs: npm install
    ↓
npm installs packages
    ↓
postinstall hook runs confrisk-npm
    ↓
Security scan executes
    ↓
If vulnerabilities found → Exit code 1 → Install fails
If clean → Exit code 0 → Success
```

---

## 🚀 Next Steps

1. **Try postinstall hook**
   ```bash
   npm install --save-dev confrisk-npm
   # Add to package.json scripts
   ```

2. **Set up git hooks**
   ```bash
   npx husky add .husky/pre-commit "confrisk-npm --exit-code"
   ```

3. **Publish to npm registry**
   - See [PUBLISHING.md](PUBLISHING.md)

4. **Share with team**
   - Commit `.husky/` and `package.json`
   - Everyone gets automatic security scanning!

---

**Version:** 0.2.0
**Status:** ✅ Production Ready
**Integration Time:** 5 minutes
**Security Improvement:** 🚀 Massive!
