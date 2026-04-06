# docs/specs/wire-check-mvp.md -- MVP Specification

## What it does

The original specification for wire-check's minimum viable product. Defines all types, function signatures, config format, CLI interface, layer behaviors, file layout, and dependencies. Describes the tool as a port of `/usr/local/bin/wire-phase-gate.sh` to a standalone Rust CLI crate, modeled after doc-drift's structure. (Source: `docs/specs/wire-check-mvp.md`)

## Key content

- Full type definitions for `Layer`, `Diagnostic`, `ConfigError`, `GitError`, `CargoError`, `LayerError`, `Config`, `DeadCodeWarning`, `OutputFormat`.
- Function signatures for all public APIs across all modules.
- CLI argument definitions matching the final clap implementation.
- Config file format with all three sections.
- Layer behavior descriptions derived from the original shell script.
- Dependency list matching the final `Cargo.toml`.

(Source: `docs/specs/wire-check-mvp.md`)

## FACTS

- The spec includes an `exclude_paths` field in `FiltersConfig` that was later removed (per `fix-review-findings.md`).
- The spec defines `Diagnostic` with a `layer: Layer` field, but the implementation uses external `(Layer, LayerResult)` tuples instead.
- The spec names `format_diagnostics` but the implementation uses `format`.
- The original tool was a shell script (`wire-phase-gate.sh`).

## INFERENCES

- The implementation diverged from the spec in minor ways (field placement, function naming) during the review/fix cycle, which is healthy.

## OPEN QUESTIONS

- None identified.

## Cross-references

- [Four layers concept](../concepts/four-layers.md)
- [Fix review findings summary](fix-review-findings.md)
- [doc-drift entity](../entities/doc-drift.md)
