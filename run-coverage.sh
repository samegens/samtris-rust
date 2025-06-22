#!/bin/bash

set -euo pipefail

if ! command -v cargo-llvm-cov &> /dev/null; then
    cargo install cargo-llvm-cov
else
    echo "cargo-llvm-cov already installed"
fi

COMMON_ARGS="--ignore-filename-regex=sdl_display\.rs|main\.rs --all-features --workspace --show-missing-lines"

cargo llvm-cov $COMMON_ARGS \
  --codecov \
  --output-path codecov.json \
  --fail-under-lines 90

cargo llvm-cov $COMMON_ARGS --html
