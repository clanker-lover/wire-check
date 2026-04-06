# src/config.rs -- Configuration Loading

## What it does

Loads and parses `wire-check.toml` configuration files using serde and the toml crate. Defines all configuration structs. When no config file exists, returns sensible defaults: all layers enabled, auto-detect crate root and base ref. Also provides crate root auto-detection (tries `src/main.rs` first, then `src/lib.rs`). (Source: `src/config.rs`, `docs/modules/config.md`)

## Public API

- `load(path: &Path) -> Result<Config, ConfigError>` -- load from TOML file. (Source: `src/config.rs:93-99`)
- `defaults() -> Config` -- all layers enabled, auto-detect everything. (Source: `src/config.rs:103-109`)
- `detect_crate_root(project_root: &Path) -> Option<PathBuf>` -- finds main.rs or lib.rs. (Source: `src/config.rs:112-122`)

## Key types

- `Config` -- top-level: `project: ProjectConfig`, `layers: LayersConfig`, `filters: FiltersConfig`. (Source: `src/config.rs:8-16`)
- `ProjectConfig` -- `crate_root: Option<PathBuf>`, `base_ref: String` (default "auto"). (Source: `src/config.rs:19-27`)
- `LayersConfig` -- four booleans, all default true. (Source: `src/config.rs:30-39`)
- `FiltersConfig` -- `test_modules: Vec<String>`, `test_files: Vec<PathBuf>`. (Source: `src/config.rs:43-51`)
- `ConfigError` -- enum: `NotFound`, `Read`, `Parse`. Uses `thiserror`. (Source: `src/config.rs:82-90`)

## Dependencies

- `serde` (derive) for deserialization
- `toml` for TOML parsing
- `thiserror` for error types

(Source: `src/config.rs`, `Cargo.toml`)

## Architecture role

Called from `main.rs` during startup. Provides configuration to all layers. No other module imports config directly -- main.rs passes config values as function arguments. (Source: `src/main.rs:64-78`)

## Tests (7 tests)

- `load_valid_config` -- full config with all sections
- `load_with_all_defaults` -- empty file gives all defaults
- `defaults_returns_all_layers_enabled` -- programmatic defaults
- `load_nonexistent_file` -- returns `ConfigError::NotFound`
- `load_invalid_toml` -- returns `ConfigError::Parse`
- `detect_crate_root_finds_main` -- prefers main.rs
- `detect_crate_root_falls_back_to_lib` -- uses lib.rs when no main.rs
- `detect_crate_root_returns_none_when_empty` -- empty dir returns None

(Source: `src/config.rs:124-227`)

## FACTS

- All config fields use serde `default` attributes, making the entire file optional.
- `base_ref` defaults to "auto" (not "main" or "master").
- `detect_crate_root` checks `main.rs` before `lib.rs` -- binary crates take priority.

## INFERENCES

- The config design prioritizes zero-config usage. A project with no `wire-check.toml` gets identical behavior to one with an empty file.

## OPEN QUESTIONS

- None identified.

## Cross-references

- [main.rs summary](main-rs.md) -- consumer of config
- [Configuration design concept](../concepts/zero-config-defaults.md)
