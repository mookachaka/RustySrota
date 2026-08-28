# RustySrota

Computational data architecture based on ternary logic and semantic flow. Grounding the math for glyph system A0-E0.

- [View the Math Derivation here](DERIVATION.md)
- [View Project Status here](STATUS.md)

## Quick Start

**Prerequisites:** Rust toolchain (install via [rustup](https://rustup.rs/) if needed)

### Clone and Build

```bash
git clone https://github.com/mookachaka/RustySrota.git
cd RustySrota
cargo build --release
```

## Overview

RustySrota provides utilities and reference implementations for:
- Geometric primitives for tensegrity and truss layouts
- Form-finding helpers and force-density solvers
- Structural member models and simple buckling checks
- CSV/JSON IO for node/edge export and quick visualization

This project is modular and intended for research prototypes, visualization, and CAD tool backends.

## Usage

```bash
cargo run --release
```

For more details, see [STATUS.md](STATUS.md) and [DERIVATION.md](DERIVATION.md).
