#!/bin/bash
# Install confrisk and confrisk-npm from GitHub releases

set -e

# Configuration
VERSION="${CONFRISK_VERSION:-latest}"
REPO="yourusername/confrisk"  # TODO: Change to your GitHub username
BASE_URL="https://github.com/${REPO}/releases"
INSTALL_DIR="${INSTALL_DIR:-/usr/local/bin}"
CONFIG_DIR="${CONFIG_DIR:-/etc/confrisk}"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Detect architecture
ARCH=$(uname -m)
case $ARCH in
    x86_64)
        ARCH="x86_64"
        ;;
    aarch64|arm64)
        ARCH="aarch64"
        ;;
    *)
        echo -e "${RED}❌ Unsupported architecture: $ARCH${NC}"
        exit 1
        ;;
esac

# Detect OS
OS=$(uname -s)
if [ "$OS" != "Linux" ]; then
    echo -e "${RED}❌ This script only supports Linux${NC}"
    echo "Detected OS: $OS"
    exit 1
fi

echo -e "${GREEN}🔒 Installing confrisk${NC}"
echo "  Version: $VERSION"
echo "  Architecture: $ARCH"
echo "  Install directory: $INSTALL_DIR"
echo "  Config directory: $CONFIG_DIR"
echo ""

# Determine download URL
if [ "$VERSION" = "latest" ]; then
    DOWNLOAD_URL="${BASE_URL}/latest/download/confrisk-linux-${ARCH}.tar.gz"
else
    DOWNLOAD_URL="${BASE_URL}/download/v${VERSION}/confrisk-${VERSION}-linux-${ARCH}.tar.gz"
fi

echo -e "${YELLOW}📥 Downloading from: $DOWNLOAD_URL${NC}"

# Create temp directory
TMP_DIR=$(mktemp -d)
cd "$TMP_DIR"

# Download
if ! curl -L "$DOWNLOAD_URL" -o confrisk.tar.gz; then
    echo -e "${RED}❌ Download failed!${NC}"
    echo "URL: $DOWNLOAD_URL"
    echo ""
    echo "Available releases: ${BASE_URL}"
    rm -rf "$TMP_DIR"
    exit 1
fi

# Extract
echo -e "${YELLOW}📦 Extracting...${NC}"
tar xzf confrisk.tar.gz

# Find extracted directory
EXTRACTED_DIR=$(find . -maxdepth 1 -type d -name "confrisk-*" | head -1)
if [ -z "$EXTRACTED_DIR" ]; then
    echo -e "${RED}❌ Extraction failed!${NC}"
    rm -rf "$TMP_DIR"
    exit 1
fi

cd "$EXTRACTED_DIR"

# Check if we need sudo
NEED_SUDO=""
if [ ! -w "$INSTALL_DIR" ]; then
    NEED_SUDO="sudo"
    echo -e "${YELLOW}⚠️  Need sudo for installation to $INSTALL_DIR${NC}"
fi

# Install binaries
echo -e "${YELLOW}🔧 Installing binaries...${NC}"
$NEED_SUDO cp confrisk "$INSTALL_DIR/"
$NEED_SUDO cp confrisk-npm "$INSTALL_DIR/"
$NEED_SUDO chmod +x "$INSTALL_DIR/confrisk" "$INSTALL_DIR/confrisk-npm"

# Install config files
if [ -d "config" ]; then
    echo -e "${YELLOW}⚙️  Installing config files...${NC}"
    $NEED_SUDO mkdir -p "$CONFIG_DIR"
    $NEED_SUDO cp -r config/* "$CONFIG_DIR/"
fi

# Cleanup
cd /
rm -rf "$TMP_DIR"

echo ""
echo -e "${GREEN}✅ confrisk installed successfully!${NC}"
echo ""
echo "Installed binaries:"
echo "  - $INSTALL_DIR/confrisk"
echo "  - $INSTALL_DIR/confrisk-npm"
echo ""
echo "Config directory:"
echo "  - $CONFIG_DIR"
echo ""
echo "Try it out:"
echo "  confrisk --help"
echo "  confrisk-npm --help"
echo ""
echo "Example usage:"
echo "  confrisk --asset production"
echo "  confrisk-npm --path /path/to/npm/project"
echo ""
echo "Documentation:"
echo "  https://github.com/${REPO}"
