#!/usr/bin/env bash
# Safe npm install wrapper with security scanning
# Usage: npm-install-safe [npm install arguments]

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}🔒 Safe npm install with confrisk-npm${NC}"
echo ""

# Parse arguments
FAIL_ON="${CONFRISK_FAIL_ON:-high}"
NPM_ARGS="$@"

# Default to 'install' if no args
if [ -z "$NPM_ARGS" ]; then
    NPM_ARGS="install"
fi

echo "Step 1: Running npm ${NPM_ARGS}..."
npm $NPM_ARGS

echo ""
echo "Step 2: Running security scan..."
echo ""

# Run confrisk-npm after installation
if command -v confrisk-npm >/dev/null 2>&1; then
    if confrisk-npm --path . --fail-on "$FAIL_ON" --exit-code; then
        echo ""
        echo -e "${GREEN}✅ Security scan passed!${NC}"
        echo ""
        exit 0
    else
        EXIT_CODE=$?
        echo ""
        echo -e "${RED}❌ Security scan failed!${NC}"
        echo -e "${RED}Found vulnerabilities with severity: $FAIL_ON or higher${NC}"
        echo ""
        echo "To fix:"
        echo "  1. Review the vulnerabilities above"
        echo "  2. Update vulnerable packages: npm update"
        echo "  3. Or remove vulnerable packages: npm uninstall <package>"
        echo ""
        echo "To bypass this check (NOT RECOMMENDED):"
        echo "  CONFRISK_SKIP=1 npm install"
        echo ""
        exit $EXIT_CODE
    fi
else
    echo -e "${YELLOW}⚠️  confrisk-npm not installed, skipping security scan${NC}"
    echo "Install: npm install -g confrisk-npm"
    echo ""
    exit 0
fi
