#!/bin/bash

set -euo pipefail

cargo clippy --all-targets --all-features -- -D warnings
