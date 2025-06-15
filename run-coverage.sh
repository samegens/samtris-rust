#!/bin/bash

set -euo pipefail

if ! command -v cargo-tarpaulin &> /dev/null; then
    cargo install cargo-tarpaulin
else
    echo "cargo-tarpaulin already installed"
fi

cargo tarpaulin --verbose --all-features --workspace --timeout 120 --out Xml --out Html
