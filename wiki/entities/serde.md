# Entity: serde and serde_json

## What they are

serde is Rust's standard serialization/deserialization framework. serde_json is its JSON format implementation.

## How wire-check uses them

**serde (with derive feature):** Used in `config.rs` for deserializing `wire-check.toml` configuration files. All config structs (`Config`, `ProjectConfig`, `LayersConfig`, `FiltersConfig`) derive `serde::Deserialize`. The `#[serde(default)]` and `#[serde(default = "...")]` attributes provide zero-config defaults.

(Source: `src/config.rs:8-51`)

**serde_json:** Used in two places:
1. `cargo.rs` -- parsing `cargo check --message-format=json` output line-by-line
2. `report.rs` -- generating JSON output format via `serde_json::json!()` macro and `to_string_pretty()`

(Source: `src/cargo.rs:47-48`, `src/report.rs:67-101`)

## Versions

- serde 1 with features: `["derive"]`
- serde_json 1

(Source: `Cargo.toml:26-27`)

## Cross-references

- [config.rs summary](../summaries/config-rs.md)
- [cargo.rs summary](../summaries/cargo-rs.md)
- [report.rs summary](../summaries/report-rs.md)
