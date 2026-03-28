# config

## What it does

Loads and parses `wire-check.toml` configuration files. Defines all config
structs: `Config`, `ProjectConfig`, `LayersConfig`, `FiltersConfig`. When no
config file exists, returns sensible defaults (all layers enabled, auto-detect
crate root and base ref). Also provides `detect_crate_root` which finds
`src/main.rs` or `src/lib.rs` automatically.

## Key types

- `Config` — top-level config with project, layers, and filters sections
- `ConfigError` — not found, read failure, or TOML parse failure

## Dependencies

Uses `serde` and `toml` for deserialization. `thiserror` for error types.
