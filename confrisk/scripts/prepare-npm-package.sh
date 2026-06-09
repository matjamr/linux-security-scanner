#!/bin/bash
# Prepare npm package for local installation

set -e

echo "🔨 Preparing confrisk-npm package..."

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Error: Run this script from the confrisk directory"
    exit 1
fi

# Build the binary
echo "Step 1: Building binary..."
cargo build --release --bin confrisk-npm

# Check if binary exists
if [ ! -f "target/release/confrisk-npm" ]; then
    echo "❌ Error: Binary not found at target/release/confrisk-npm"
    exit 1
fi

# Create npm-package directory
mkdir -p npm-package

# Copy binary
echo "Step 2: Copying binary..."
cp target/release/confrisk-npm npm-package/

# Copy README
echo "Step 3: Copying documentation..."
if [ -f "CONFRISK_NPM.md" ]; then
    cp CONFRISK_NPM.md npm-package/README.md
else
    echo "Warning: CONFRISK_NPM.md not found, creating basic README"
    cat > npm-package/README.md << 'EOF'
# confrisk-npm

NPM dependency security scanner with risk-based scoring.

## Usage

```bash
confrisk-npm --path /path/to/project
confrisk-npm --fail-on high --exit-code
```

## Documentation

See https://github.com/yourusername/confrisk
EOF
fi

# Make binary executable
chmod +x npm-package/confrisk-npm

# Verify package.json exists
if [ ! -f "npm-package/package.json" ]; then
    echo "❌ Error: npm-package/package.json not found"
    exit 1
fi

echo ""
echo "✅ Package prepared successfully!"
echo ""
echo "📦 Package location: npm-package/"
echo ""
echo "To install locally:"
echo "  cd npm-package"
echo "  npm link"
echo ""
echo "Or in your project:"
echo "  npm install /path/to/confrisk/npm-package"
echo ""
