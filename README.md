# SAMTris Rust

[![CI](https://github.com/samegens/samtris-rust/actions/workflows/ci.yml/badge.svg)](https://github.com/samegens/samtris-rust/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/samegens/samtris-rust/branch/main/graph/badge.svg)](https://codecov.io/gh/samegens/samtris-rust)

A Tetris clone implemented in Rust using SDL2 using TDD and test coverage.

## Prerequisites

- Rust 1.70.0 or later (check with ```rustc --version```)

### Ubuntu/Debian

```bash
sudo apt-get update
sudo apt-get install -y libsdl2-dev libsdl2-image-dev libsdl2-ttf-dev
```

## TODO

- Add start screen
- Add high score screen
- Find out if an integration test can be written that covers main and SdlDisplay.
