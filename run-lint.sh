#!/bin/bash

set -euo pipefail

echo "=== Tool Versions ==="
echo "Rust: $(rustc --version)"
echo "Cargo: $(cargo --version)"
echo "Clippy: $(cargo clippy --version)"
echo "Rustfmt: $(rustfmt --version)"
echo "=== End Tool Versions ==="
echo

cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings -A dead_code -A clippy::module-inception
