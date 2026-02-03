#!/usr/bin/env bash
# Run the SYM test harness

set -e

cd "$(dirname "$0")"

echo "Building test harness..."
cargo build --release

echo ""
echo "Running tests..."
cargo run --release
