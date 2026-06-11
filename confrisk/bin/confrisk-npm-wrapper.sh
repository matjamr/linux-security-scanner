#!/usr/bin/env bash
# confrisk-npm wrapper script
# Automatically finds and executes the confrisk-npm binary

set -e

# Find the binary location
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BINARY_PATHS=(
    # Installed via npm (local)
    "$SCRIPT_DIR/../target/release/confrisk-npm"
    # Installed via npm (global)
    "$SCRIPT_DIR/confrisk-npm"
    # System installation
    "/usr/local/bin/confrisk-npm"
    "/usr/bin/confrisk-npm"
    # Current directory build
    "$(pwd)/target/release/confrisk-npm"
)

# Find the first available binary
BINARY=""
for path in "${BINARY_PATHS[@]}"; do
    if [ -x "$path" ]; then
        BINARY="$path"
        break
    fi
done

if [ -z "$BINARY" ]; then
    echo "Error: confrisk-npm binary not found!"
    echo "Please build it with: cargo build --release --bin confrisk-npm"
    exit 1
fi

# Execute the binary with all arguments
exec "$BINARY" "$@"
