# Entity: clap

## What it is

A Rust command-line argument parsing library. wire-check uses clap 4 with the `derive` feature, which allows defining CLI arguments as a struct with derive macros rather than a builder API.

## How wire-check uses it

The `Cli` struct in `src/main.rs` derives `clap::Parser`. Fields with `#[arg(...)]` attributes define the CLI interface: `project_root` (positional, default "."), `--config`, `--format`, `--base-ref`, `--layer`. The `#[command(version, about)]` attribute pulls version and description from `Cargo.toml`.

(Source: `src/main.rs:15-38`, `Cargo.toml:25`)

## Version

clap 4 with features: `["derive"]`. (Source: `Cargo.toml:25`)

## MSRV impact

clap 4's minimum supported Rust version influenced wire-check's `rust-version = "1.74.0"` setting. (Source: `docs/specs/fix-review-findings.md:44-47`)

## Cross-references

- [main.rs summary](../summaries/main-rs.md)
- [Cargo.toml summary](../summaries/cargo-toml.md)
