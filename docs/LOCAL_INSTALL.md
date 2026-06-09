# Local Installation Guide

## How to Install confrisk-npm Locally (Without npmjs.com)

Since confrisk-npm is not yet published to npmjs.com, use one of these methods to install it locally:

---

## 🚀 Method 1: npm link (Recommended for Development)

**Best for:** Active development, testing across multiple projects

### Step 1: Build and Link

```bash
cd /path/to/confrisk

# Prepare the package (builds binary + copies files)
./scripts/prepare-npm-package.sh

# Create global link
cd npm-package
npm link
```

### Step 2: Use in Your Project

```bash
cd /path/to/your/project

# Link the package
npm link confrisk-npm

# Verify installation
confrisk-npm --help
```

### Step 3: Add to package.json

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

### Unlink When Done

```bash
# In your project
npm unlink confrisk-npm

# Remove global link
cd /path/to/confrisk/npm-package
npm unlink -g
```

---

## 📦 Method 2: Install from Local Directory

**Best for:** Quick testing, one-off installations

### Option A: Direct Install

```bash
cd /path/to/your/project

# Build the package first
cd /path/to/confrisk
./scripts/prepare-npm-package.sh

# Install from local directory
cd /path/to/your/project
npm install /path/to/confrisk/npm-package
```

### Option B: Using file: Protocol

**package.json:**
```json
{
  "devDependencies": {
    "confrisk-npm": "file:../confrisk/npm-package"
  }
}
```

Then:
```bash
npm install
```

**Pros:**
- Survives `npm install`
- Works with package-lock.json
- Team members see relative path

**Cons:**
- Path is relative to project
- Binary not updated automatically

---

## 🔗 Method 3: Global Binary Install

**Best for:** Using as a CLI tool, not as npm dependency

```bash
# Build the binary
cd /path/to/confrisk
cargo build --release --bin confrisk-npm

# Install globally
sudo cp target/release/confrisk-npm /usr/local/bin/

# Verify
confrisk-npm --help
```

Now use in any project:

**package.json:**
```json
{
  "scripts": {
    "postinstall": "confrisk-npm --fail-on high --exit-code"
  }
}
```

**Note:** Doesn't need `devDependencies` entry since binary is globally available.

---

## ⚡ Quick Setup Script

Create this helper script:

**install-confrisk-npm.sh:**
```bash
#!/bin/bash
set -e

CONFRISK_PATH="/path/to/confrisk"

echo "🔨 Building confrisk-npm..."
cd "$CONFRISK_PATH"
cargo build --release --bin confrisk-npm

echo "📦 Preparing npm package..."
./scripts/prepare-npm-package.sh

echo "🔗 Creating global link..."
cd npm-package
npm link

echo "✅ Done! Now you can use:"
echo "  npm link confrisk-npm    (in your project)"
```

Make it executable:
```bash
chmod +x install-confrisk-npm.sh
./install-confrisk-npm.sh
```

---

## 🧪 Testing the Installation

### Test 1: CLI Works

```bash
confrisk-npm --help
# Should show usage information
```

### Test 2: Scan a Project

```bash
cd /path/to/your/npm/project
confrisk-npm --path .
```

### Test 3: postinstall Hook

Create test project:

```bash
mkdir test-project
cd test-project
npm init -y
npm link confrisk-npm
```

**package.json:**
```json
{
  "scripts": {
    "postinstall": "confrisk-npm --fail-on high --exit-code"
  }
}
```

Install a vulnerable package:
```bash
npm install lodash@4.17.20
# Should fail with security error!
```

---

## 🔄 Updating After Changes

### If you modified the Rust code:

```bash
cd /path/to/confrisk

# Rebuild
cargo build --release --bin confrisk-npm

# Update package
./scripts/prepare-npm-package.sh

# npm link automatically uses the updated binary
```

### Force Update in Projects

```bash
cd /path/to/your/project

# Unlink and relink
npm unlink confrisk-npm
npm link confrisk-npm
```

---

## 🐛 Troubleshooting

### "confrisk-npm: command not found"

**Solution 1:** Check npm link
```bash
npm list -g --depth=0 | grep confrisk-npm
```

**Solution 2:** Re-link
```bash
cd /path/to/confrisk/npm-package
npm unlink -g
npm link
```

### "Binary not found" error in postinstall

**Solution:** Use absolute path or global install
```json
{
  "scripts": {
    "postinstall": "/usr/local/bin/confrisk-npm --fail-on high --exit-code"
  }
}
```

### npm link not working

**Solution:** Use direct path install
```bash
npm install file:///absolute/path/to/confrisk/npm-package
```

### "Permission denied"

**Solution 1:** Use sudo (not recommended)
```bash
sudo npm link
```

**Solution 2:** Fix npm permissions
```bash
mkdir ~/.npm-global
npm config set prefix '~/.npm-global'
echo 'export PATH=~/.npm-global/bin:$PATH' >> ~/.bashrc
source ~/.bashrc
```

---

## 📋 Complete Example Workflow

### 1. Initial Setup (One Time)

```bash
# In confrisk repo
cd /path/to/confrisk
./scripts/prepare-npm-package.sh
cd npm-package
npm link
```

### 2. Use in Project A

```bash
cd /path/to/project-a
npm link confrisk-npm

# Add to package.json:
{
  "scripts": {
    "postinstall": "confrisk-npm --fail-on high --exit-code"
  }
}

# Test it
npm install lodash@4.17.20  # Should fail!
```

### 3. Use in Project B

```bash
cd /path/to/project-b
npm link confrisk-npm

# Works immediately!
confrisk-npm --path .
```

### 4. Update confrisk-npm

```bash
# Make changes to Rust code
cd /path/to/confrisk
vim src/npm.rs

# Rebuild
cargo build --release --bin confrisk-npm
./scripts/prepare-npm-package.sh

# All linked projects now use updated version!
```

---

## 🎯 Recommended Setup

For active development:

```bash
# 1. Build and link once
cd /path/to/confrisk
./scripts/prepare-npm-package.sh
cd npm-package
npm link

# 2. In each project where you want to use it:
cd /path/to/your/project
npm link confrisk-npm
```

For production use (when ready):

```bash
# Publish to npmjs.com (one time)
cd /path/to/confrisk/npm-package
npm login
npm publish

# Then users can:
npm install --save-dev confrisk-npm
```

---

## 📊 Comparison

| Method | Updates Automatically | Survives npm install | Works Across Projects |
|--------|----------------------|---------------------|----------------------|
| **npm link** | ✅ Yes | ✅ Yes | ✅ Yes |
| **file: path** | ❌ No | ✅ Yes | ❌ No |
| **Direct install** | ❌ No | ❌ No | ❌ No |
| **Global binary** | ❌ No | ✅ Yes | ✅ Yes |

**Recommendation:** Use **npm link** for development!

---

## ✅ Verification Checklist

- [ ] Binary builds: `cargo build --release --bin confrisk-npm`
- [ ] Package prepared: `./scripts/prepare-npm-package.sh`
- [ ] Global link created: `npm link` (in npm-package/)
- [ ] Command works: `confrisk-npm --help`
- [ ] Project linked: `npm link confrisk-npm` (in your project)
- [ ] postinstall works: `npm install` triggers scan
- [ ] Fails on vulnerabilities: `npm install lodash@4.17.20` blocks

---

## 🎓 Next Steps

1. **Test the setup:**
   ```bash
   cd examples/npm-postinstall-demo
   npm link confrisk-npm
   npm install lodash@4.17.20  # Should fail!
   ```

2. **Use in your projects:**
   ```bash
   npm link confrisk-npm
   # Add postinstall to package.json
   ```

3. **When ready to publish:**
   See [PUBLISHING.md](PUBLISHING.md) for npm publish guide

---

**Current Status:**
- ✅ Binary: Built and ready
- ✅ Package: Prepared in `npm-package/`
- ✅ Global link: Created with `npm link`
- ✅ Ready to use: `npm link confrisk-npm` in any project

**Quick Test:**
```bash
confrisk-npm --help
# Should work! ✅
```
