# Concept: Zero-Config Defaults

## What it is

wire-check is designed to work with no configuration file at all. Every setting has a sensible default, making the tool usable by running just `wire-check` in any Rust project directory.

## Default behavior

| Setting | Default | Source |
|---------|---------|--------|
| Project root | Current directory | CLI default `"."` |
| Config file | `<project_root>/wire-check.toml` | `main.rs:65-66` |
| Crate root | Auto-detect: `src/main.rs` then `src/lib.rs` | `config.rs:112-122` |
| Base ref | Auto-detect: tries "main", then "master", falls back to "HEAD~1" | `git.rs:16-34` |
| All four layers | Enabled | `config.rs:31-38` |
| Test modules | Empty list | `config.rs:46` |
| Test files | Empty list | `config.rs:49` |
| Output format | Human-readable | CLI default `"human"` |

(Source: `src/main.rs`, `src/config.rs`, `src/git.rs`)

## Implementation details

All config structs use serde's `#[serde(default)]` and `#[serde(default = "...")]` attributes. This means:

- An empty `wire-check.toml` file is valid (all defaults apply)
- A missing `wire-check.toml` file is valid (programmatic defaults via `config::defaults()`)
- Partial config files are valid (only specified fields override defaults)

(Source: `src/config.rs:8-51`)

## Auto-detection chain

The tool auto-detects two things:

1. **Crate root:** `detect_crate_root()` checks for `src/main.rs` first (binary crate), then `src/lib.rs` (library crate). Binary crates take priority.
2. **Base ref:** `detect_base_ref()` tries `git rev-parse --verify main`, then `master`, falls back to `HEAD~1`. Covers the most common branch naming conventions.

Both can be overridden via config or CLI flags.

(Source: `src/config.rs:112-122`, `src/git.rs:16-34`)

## Cross-references

- [config.rs summary](../summaries/config-rs.md)
- [git.rs summary](../summaries/git-rs.md)
- [main.rs summary](../summaries/main-rs.md)
