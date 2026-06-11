# npm postinstall Hook Demo

## Demo: Automatic Security Scanning on `npm install`

This demo shows how `confrisk-npm` automatically scans dependencies after installation using the `postinstall` hook.

## How It Works

When you run `npm install`, the following happens:

```
1. npm installs packages
2. postinstall hook triggers
3. confrisk-npm scans all dependencies
4. If vulnerabilities found → EXIT CODE 1 (fails)
5. If clean → EXIT CODE 0 (success)
```

## Try It

### Scenario 1: Clean Installation (Should Pass)

```bash
cd examples/npm-postinstall-demo

# Install safe package
npm install

# Output:
# > npm-postinstall-demo@1.0.0 postinstall
# > confrisk-npm --path . --config ../../config --fail-on high --exit-code
#
# No security issues found!
```

**Result:** Installation succeeds

### Scenario 2: Install Vulnerable Package (Should Fail)

```bash
# Add vulnerable lodash version
npm install lodash@4.17.20

# Output:
# > npm-postinstall-demo@1.0.0 postinstall
# > confrisk-npm --path . --config ../../config --fail-on high --exit-code
#
# [NPM-BLOCKED-LODASH] Blocked package: lodash
#    Package:  lodash@4.17.20
#    Reason:   Prototype pollution vulnerabilities < 4.17.21
#    Severity: HIGH
#
# Security scan failed!
# npm ERR! Exit code: 1
```

**Result:** postinstall hook fails, npm shows error

### Scenario 3: Fix the Vulnerability

```bash
# Update to safe version
npm install lodash@latest

# Output:
# No security issues found!
```

**Result:** Installation succeeds with safe version

## Configuration

### package.json

```json
{
  "scripts": {
    "postinstall": "confrisk-npm --path . --config ../../config --fail-on high --exit-code"
  }
}
```

**Explanation:**
- `--path .` - Scan current directory
- `--config ../../config` - Use confrisk config
- `--fail-on high` - Fail on HIGH or CRITICAL vulnerabilities
- `--exit-code` - Exit with code 1 if vulnerabilities found

## Adjust Severity Level

Edit `package.json` to change when it fails:

```json
{
  "scripts": {
    "postinstall": "confrisk-npm --fail-on critical --exit-code"
  }
}
```

Options:
- `critical` - Only fail on CRITICAL vulnerabilities
- `high` - Fail on HIGH or CRITICAL
- `medium` - Fail on MEDIUM, HIGH, or CRITICAL
- `low` - Fail on any vulnerability

## Use in Your Project

Copy this pattern to your own project:

```bash
# 1. Build confrisk-npm
cd ../../
cargo build --release --bin confrisk-npm

# 2. Add to your package.json
{
  "scripts": {
    "postinstall": "/path/to/confrisk-npm --fail-on high --exit-code"
  }
}

# 3. Or install globally
sudo cp target/release/confrisk-npm /usr/local/bin/
# Then in package.json:
{
  "scripts": {
    "postinstall": "confrisk-npm --fail-on high --exit-code"
  }
}
```

## Skip Security Check

Sometimes you need to bypass (not recommended):

```bash
# Skip all npm scripts
npm install --ignore-scripts

# Or skip just postinstall
npm install --ignore-scripts && npm run postinstall || true
```

## See Also

- [CONFRISK_NPM.md](../../CONFRISK_NPM.md) - Full npm scanner documentation
