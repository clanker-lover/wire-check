# src/main.rs -- Binary Entrypoint

## What it does

Binary entrypoint for the wire-check CLI. Parses command-line arguments via clap, loads configuration, resolves the crate root and git base ref, runs the four verification layers, formats output, and exits with the appropriate code. Contains no business logic -- it is purely an orchestrator that wires modules together. (Source: `src/main.rs`, `docs/ARCHITECTURE.md`)

## Public API

This is the binary crate root. It declares five internal modules: `cargo`, `config`, `git`, `layers`, `report`. (Source: `src/main.rs:3-7`)

## Key types and functions

- `Cli` struct (clap derive): fields `project_root` (default "."), `config` (optional path), `format` ("human"/"json"), `base_ref` (optional override), `layer` (optional single-layer filter). (Source: `src/main.rs:18-38`)
- `main()`: the entire function. Handles config loading, crate root resolution, base ref resolution, layer execution, reporting, and exit codes. (Source: `src/main.rs:40-187`)

## Dependencies

Imports from: `clap::Parser`, `layers::{Layer, LayerResult}`, `report::OutputFormat`, `config`, `git`. (Source: `src/main.rs`)

## Architecture role

The orchestration point. Every module feeds into `main.rs` or is called from it. The `should_run` closure selects layers based on CLI `--layer` flag and config toggles. Results are collected as `Vec<(Layer, LayerResult)>` and passed to `report::format()`. (Source: `src/main.rs:122-179`, `docs/ARCHITECTURE.md`)

## Exit codes

- 0: all checks pass
- 1: at least one diagnostic failure
- 2: configuration or runtime error (bad path, unknown format, git/cargo failure)

(Source: `src/main.rs`, `README.md`)

## FACTS

- Contains zero business logic per architectural invariant.
- Uses `process::exit()` for all error paths rather than `Result`-based propagation.
- Does not print to stdout directly except through `report::format()`.

## INFERENCES

- The `process::exit(2)` pattern in `main()` means errors short-circuit before reaching the reporting layer. This is intentional -- config/runtime errors are not diagnostics.

## OPEN QUESTIONS

- Why does `main()` use `eprintln!` + `process::exit(2)` rather than returning `Result` from `main`? Likely a deliberate choice to keep error messages simple without anyhow's formatting, but `anyhow` is listed as a dependency and unused in this file.

## Cross-references

- [config.rs summary](config-rs.md) -- config loading
- [git.rs summary](git-rs.md) -- base ref detection
- [layers.rs summary](layers-rs.md) -- the four checks
- [report.rs summary](report-rs.md) -- output formatting
- [Orchestrator pattern concept](../concepts/orchestrator-pattern.md)
- [Four layers concept](../concepts/four-layers.md)
