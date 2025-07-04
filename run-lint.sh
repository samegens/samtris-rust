#!/bin/bash

set -euo pipefail

cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings -A dead_code -A clippy::module-inception
