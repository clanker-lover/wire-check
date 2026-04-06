# Entity: GitHub Actions

## What it is

GitHub's CI/CD platform used by wire-check for automated testing.

## How wire-check uses it

A single workflow file (`.github/workflows/ci.yml`) runs on push to master and pull requests. It uses three third-party actions:

- `actions/checkout@v4` -- repository checkout
- `dtolnay/rust-toolchain@stable` -- Rust toolchain installation (by dtolnay, a prolific Rust ecosystem maintainer and author of thiserror, anyhow, serde, syn)
- `Swatinem/rust-cache@v2` -- Cargo build cache for faster CI

The pipeline runs: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`.

(Source: `.github/workflows/ci.yml`)

## Cross-references

- [ci-yml summary](../summaries/ci-yml.md)
- [Development methodology concept](../concepts/development-methodology.md)
