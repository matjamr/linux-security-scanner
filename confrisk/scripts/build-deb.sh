#!/bin/bash
# Build .deb packages for confrisk and confrisk-npm

set -e

echo "🔨 Building .deb packages for confrisk..."

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Error: Run this script from the confrisk directory"
    exit 1
fi

# Create debian directory structure
echo "📁 Creating debian package structure..."
mkdir -p debian/confrisk/DEBIAN
mkdir -p debian/confrisk/usr/bin
mkdir -p debian/confrisk/etc/confrisk
mkdir -p debian/confrisk/usr/share/doc/confrisk

mkdir -p debian/confrisk-npm/DEBIAN
mkdir -p debian/confrisk-npm/usr/bin
mkdir -p debian/confrisk-npm/usr/share/doc/confrisk-npm

# Build binaries
echo "🦀 Building Rust binaries..."
cargo build --release --bin confrisk
cargo build --release --bin confrisk-npm

# Install confrisk files
echo "📦 Packaging confrisk..."
cp target/release/confrisk debian/confrisk/usr/bin/
chmod +x debian/confrisk/usr/bin/confrisk
cp -r config/* debian/confrisk/etc/confrisk/
cp README.md debian/confrisk/usr/share/doc/confrisk/
cp CONFIG_SYSTEM.md debian/confrisk/usr/share/doc/confrisk/

# Create confrisk control file
cat > debian/confrisk/DEBIAN/control << 'EOF'
Package: confrisk
Version: 0.2.0
Section: admin
Priority: optional
Architecture: amd64
Maintainer: confrisk developers <dev@example.com>
Description: Linux security configuration scanner with risk-based scoring
 confrisk is a security scanner that provides contextual risk assessment
 for Linux systems. Features include:
  - Risk-based prioritization
  - Config-driven security checks
  - HTML report generation
  - Explainable AI/ML scoring
Homepage: https://github.com/yourusername/confrisk
EOF

# Install confrisk-npm files
echo "📦 Packaging confrisk-npm..."
cp target/release/confrisk-npm debian/confrisk-npm/usr/bin/
chmod +x debian/confrisk-npm/usr/bin/confrisk-npm
cp CONFRISK_NPM.md debian/confrisk-npm/usr/share/doc/confrisk-npm/README.md
cp NPM_SCANNER_SUMMARY.md debian/confrisk-npm/usr/share/doc/confrisk-npm/

# Create confrisk-npm control file
cat > debian/confrisk-npm/DEBIAN/control << 'EOF'
Package: confrisk-npm
Version: 0.2.0
Section: devel
Priority: optional
Architecture: amd64
Depends: nodejs, npm
Maintainer: confrisk developers <dev@example.com>
Description: NPM dependency security scanner with git hooks integration
 A dedicated npm dependency scanner that integrates with git hooks
 and CI/CD pipelines. Features include:
  - Blocklist checking for vulnerable packages
  - npm audit integration
  - Risk-based scoring
  - JSON and text output formats
  - CI/CD exit codes
Homepage: https://github.com/yourusername/confrisk
EOF

# Build .deb packages
echo "🔧 Building .deb packages..."
dpkg-deb --build debian/confrisk
dpkg-deb --build debian/confrisk-npm

# Move to releases directory
mkdir -p releases
mv debian/confrisk.deb releases/confrisk_0.2.0_amd64.deb
mv debian/confrisk-npm.deb releases/confrisk-npm_0.2.0_amd64.deb

# Create checksums
cd releases
sha256sum confrisk_0.2.0_amd64.deb > confrisk_0.2.0_amd64.deb.sha256
sha256sum confrisk-npm_0.2.0_amd64.deb > confrisk-npm_0.2.0_amd64.deb.sha256
cd ..

# Cleanup
rm -rf debian/confrisk debian/confrisk-npm

echo ""
echo "✅ .deb packages created successfully!"
echo ""
echo "📦 Packages:"
echo "   - releases/confrisk_0.2.0_amd64.deb"
echo "   - releases/confrisk-npm_0.2.0_amd64.deb"
echo ""
echo "🧪 Test installation:"
echo "   sudo dpkg -i releases/confrisk_0.2.0_amd64.deb"
echo "   sudo dpkg -i releases/confrisk-npm_0.2.0_amd64.deb"
echo ""
echo "✅ Verify:"
echo "   confrisk --help"
echo "   confrisk-npm --help"
echo "   dpkg -L confrisk"
echo ""
echo "🗑️  Uninstall:"
echo "   sudo dpkg -r confrisk confrisk-npm"
