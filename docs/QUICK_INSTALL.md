# Quick Install Guide - confrisk-npm

## 🚀 Install Locally in 3 Commands

Since confrisk-npm is not yet on npmjs.com, use npm link:

```bash
# 1. Build and prepare the package
cd /path/to/confrisk
./scripts/prepare-npm-package.sh

# 2. Create global link
cd npm-package
npm link

# 3. Done! Now confrisk-npm is available globally
confrisk-npm --help
```

---

## 📦 Use in Your Project

### Option 1: Link to existing project

```bash
cd /path/to/your/project
npm link confrisk-npm
```

### Option 2: Add postinstall hook

**package.json:**
```json
{
  "scripts": {
    "postinstall": "confrisk-npm --fail-on high --exit-code"
  },
  "devDependencies": {
    "confrisk-npm": "*"
  }
}
```

Then:
```bash
npm link confrisk-npm
```

---

## ✅ Verify Installation

```bash
# Check command works
confrisk-npm --help

# Scan current directory
confrisk-npm --path .

# Test with vulnerable package
npm install lodash@4.17.20
# Should fail! ❌
```

---

## 🎯 Complete Example

```bash
# 1. Setup confrisk-npm (one time)
cd /path/to/confrisk
./scripts/prepare-npm-package.sh
cd npm-package
npm link

# 2. Use in your project
cd /path/to/your/project

# Link the package
npm link confrisk-npm

# Add to package.json
cat >> package.json << 'EOF'
{
  "scripts": {
    "postinstall": "confrisk-npm --fail-on high --exit-code"
  }
}
EOF

# Test it!
npm install lodash@4.17.20
# ❌ Blocked by security scan!
```

---

## 🔄 Update After Code Changes

```bash
cd /path/to/confrisk

# Rebuild
cargo build --release --bin confrisk-npm

# Update package
./scripts/prepare-npm-package.sh

# All linked projects automatically use new version! ✅
```

---

## 🐛 Troubleshooting

### "confrisk-npm: command not found"

```bash
# Re-run npm link
cd /path/to/confrisk/npm-package
npm link

# Verify
which confrisk-npm
```

### postinstall not running

```bash
# In your project
npm link confrisk-npm

# Verify link
npm list confrisk-npm
```

---

## 📖 Full Documentation

- **[LOCAL_INSTALL.md](LOCAL_INSTALL.md)** - Complete installation guide
- **[NPM_INSTALL_INTEGRATION.md](NPM_INSTALL_INTEGRATION.md)** - Integration patterns
- **[CONFRISK_NPM.md](CONFRISK_NPM.md)** - Full npm scanner docs

---

**That's it! You can now use `npm install --save-dev confrisk-npm` equivalent via npm link.** ✅
