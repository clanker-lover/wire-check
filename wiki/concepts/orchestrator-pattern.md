# Concept: Orchestrator Pattern in main.rs

## What it is

`main.rs` is a pure orchestrator: it wires modules together, handles I/O, and manages process lifecycle, but contains zero business logic. This is an architectural invariant, not a suggestion.

## How it manifests

The `main()` function follows a strict sequence:

1. Parse CLI args (clap)
2. Canonicalize project root path
3. Parse output format
4. Load config (or defaults)
5. Resolve crate root (config or auto-detect)
6. Resolve base ref (CLI override, config, or auto-detect)
7. Run each layer (via `should_run` closure checking CLI filter + config toggle)
8. Collect results as `Vec<(Layer, LayerResult)>`
9. Format output via `report::format()`
10. Exit with appropriate code

No step performs any verification logic. Each step delegates to a module.

(Source: `src/main.rs:40-187`, `docs/ARCHITECTURE.md:50-52`)

## Why it matters

- **Testability:** Each module is testable independently because it receives explicit inputs rather than reading from global state.
- **Readability:** `main.rs` reads as a high-level recipe. You can understand the tool's flow without reading any module internals.
- **Invariant enforcement:** The architecture doc states "main.rs contains no business logic" as a violation-is-a-bug invariant.

(Source: `docs/ARCHITECTURE.md:50-52`)

## Error handling at the orchestrator level

`main.rs` uses `eprintln!` + `process::exit(2)` for all error paths, not `Result`-based propagation from `main()`. This keeps error messages simple and user-readable. Each module defines its own error types with `thiserror`, but `main.rs` stringifies them via `Display`.

(Source: `src/main.rs:44-50`, `src/main.rs:69-74`)

## Cross-references

- [main.rs summary](../summaries/main-rs.md)
- [report.rs summary](../summaries/report-rs.md)
- [External command isolation concept](external-command-isolation.md)
- [Error handling concept](error-handling.md)
