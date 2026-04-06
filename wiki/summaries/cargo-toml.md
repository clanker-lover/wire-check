# Cargo.toml -- Package Manifest

## What it does

Defines the wire-check crate metadata, dependencies, and build configuration. Published to crates.io as a command-line utility. (Source: `Cargo.toml`)

## Key metadata

- **Name:** wire-check
- **Version:** 0.1.0
- **Edition:** 2021
- **Rust version:** 1.74.0 (minimum supported)
- **License:** MIT OR Apache-2.0 (dual license)
- **Repository:** https://github.com/clanker-lover/wire-check
- **Authors:** Brandon, Claude Opus 4.6
- **Keywords:** dead-code, lint, ratchet, ci, verification
- **Categories:** command-line-utilities, development-tools

(Source: `Cargo.toml:1-11`)

## Dependencies

| Crate | Version | Features | Purpose |
|-------|---------|----------|---------|
| clap | 4 | derive | CLI argument parsing |
| serde | 1 | derive | Config deserialization |
| serde_json | 1 | -- | Cargo JSON output parsing, JSON report |
| toml | 0.8 | -- | wire-check.toml config parsing |
| thiserror | 2 | -- | Error type derivation |
| anyhow | 1 | -- | Listed but not visibly used in source |

(Source: `Cargo.toml:24-31`)

## Dev dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| tempfile | 3 | Temporary directories for tests |

(Source: `Cargo.toml:33`)

## Include list

Only ships: `src/**/*`, `Cargo.toml`, `Cargo.lock`, `LICENSE-MIT`, `LICENSE-APACHE`, `README.md`, `CHANGELOG.md`. (Source: `Cargo.toml:14-22`)

## FACTS

- Uses `thiserror` v2, not v1.
- Requires Rust 1.74.0+, which matches clap 4's MSRV.
- The `include` list explicitly controls what goes to crates.io.

## INFERENCES

- `anyhow` is listed as a dependency but does not appear in any source file's imports. It may be leftover from early development or reserved for future use.

## OPEN QUESTIONS

- Is the `anyhow` dependency actually used? No `use anyhow` appears in any source file.

## Cross-references

- [clap entity](../entities/clap.md)
- [thiserror entity](../entities/thiserror.md)
- [serde entity](../entities/serde.md)
