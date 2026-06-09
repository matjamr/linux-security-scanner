#!/bin/bash

set -e

echo "Building static Linux binary for confrisk..."

# Build using Docker with Rust Alpine image
docker run --rm \
  -v "$(pwd)/..":/workspace \
  -w /workspace \
  rust:1.75-alpine \
  sh -c "apk add --no-cache musl-dev && cargo build --release --target x86_64-unknown-linux-musl"

# Copy the binary to demo directory
cp ../target/x86_64-unknown-linux-musl/release/confrisk ./confrisk

echo "Binary built and copied to demo/confrisk"
ls -lh confrisk
