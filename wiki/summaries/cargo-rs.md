# src/cargo.rs -- Cargo Check Runner and JSON Parser

## What it does

Runs `cargo check` with `--force-warn dead_code` and `--message-format=json`, then parses the JSON output line-by-line to extract only `dead_code` warnings. Each warning includes file path, message text, and line number from the primary span. All other compiler messages (unused imports, build artifacts, etc.) are filtered out. (Source: `src/cargo.rs`, `docs/modules/cargo.md`)

## Public API

- `dead_code_warnings(project_root: &Path) -> Result<Vec<DeadCodeWarning>, CargoError>` -- runs cargo and returns parsed warnings. (Source: `src/cargo.rs:24-36`)

## Key types

- `DeadCodeWarning` -- struct with `file: PathBuf`, `message: String`, `line: usize`. (Source: `src/cargo.rs:8-12`)
- `CargoError` -- enum: `CommandFailed(String)`, `ParseFailed(String)`. Uses `thiserror`. (Source: `src/cargo.rs:15-21`)

## Internal functions

- `parse_dead_code_warnings(json_output: &str) -> Result<Vec<DeadCodeWarning>, CargoError>` -- parses JSON lines, extracts primary span from `dead_code` warnings. (Source: `src/cargo.rs:39-114`)

## Dependencies

- `std::process::Command` for running cargo
- `serde_json` for JSON parsing
- `thiserror` for error derivation

(Source: `src/cargo.rs`, `docs/modules/cargo.md`)

## Architecture role

One of two modules allowed to run external commands (the other is `git.rs`). Called only by `layers.rs` Layer 3 (dead code ratchet). (Source: `docs/ARCHITECTURE.md`)

## Tests (3 tests)

- `parse_extracts_dead_code_only` -- verifies only dead_code warnings are extracted from mixed JSON output
- `parse_empty_input` -- empty string returns empty vec
- `parse_no_dead_code` -- JSON with no dead_code messages returns empty vec

(Source: `src/cargo.rs:116-146`)

## FACTS

- Uses `RUSTFLAGS=--force-warn dead_code` environment variable, not `-D` flag. This means cargo still succeeds even with warnings.
- Parses JSON line-by-line (not as a single array). Each line is an independent JSON object.
- Only extracts the primary span (`is_primary: true`) from each warning.

## INFERENCES

- The `--force-warn` approach (vs `-D`) is deliberate: it ensures cargo check completes and produces all warnings rather than stopping at the first one.

## OPEN QUESTIONS

- None identified.

## Cross-references

- [layers.rs summary](layers-rs.md) -- consumer of `dead_code_warnings()`
- [Dead code ratchet concept](../concepts/dead-code-ratchet.md)
- [External command isolation concept](../concepts/external-command-isolation.md)
