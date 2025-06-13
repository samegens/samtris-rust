#!/bin/bash

set -euo pipefail

cargo tarpaulin --out Html
