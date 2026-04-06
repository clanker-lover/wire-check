# src/report.rs -- Output Formatting

## What it does

Formats diagnostics for human-readable or JSON output. The human format shows each layer with a header, PASS/FAIL/SKIP status, and a summary line. The JSON format produces a structured object with a layers array and summary. This module never decides what to check -- it only formats what the layers found. (Source: `src/report.rs`, `docs/modules/report.md`)

## Public API

- `format(results: &[(Layer, LayerResult)], output_format: OutputFormat) -> String` -- dispatches to human or JSON formatter. (Source: `src/report.rs:13-17`)

## Key types

- `OutputFormat` -- enum: `Human`, `Json`. (Source: `src/report.rs:6-9`)

## Internal functions

- `format_human(results)` -- produces the `=== Wire Check ===` banner, per-layer sections with PASS/FAIL/SKIP, and final `WIRE CHECK PASSED` or `WIRE CHECK FAILED: N error(s)`. (Source: `src/report.rs:20-57`)
- `format_json(results)` -- produces JSON with `layers` array (each with `layer`, `status`, `diagnostics`) and `summary` object (`total`, `passed`). (Source: `src/report.rs:59-102`)
- `layer_label(layer)` -- maps enum to display name like "Layer 1: Annotation ban". (Source: `src/report.rs:104-111`)
- `layer_description(layer)` -- maps enum to pass-message like "No dead_code suppression on module declarations". (Source: `src/report.rs:113-120`)

## Dependencies

- `serde_json` for JSON serialization
- `crate::layers::{Layer, LayerResult}` for input types

(Source: `src/report.rs:1-3`)

## Architecture role

Terminal node in the data flow. Receives collected results from `main.rs`, produces a string. Never called by any other module. The architectural invariant states that formatting is report.rs's sole responsibility -- no other module prints to stdout. (Source: `docs/ARCHITECTURE.md`)

## Tests (4 tests)

- `human_format_all_pass` -- verifies "WIRE CHECK PASSED" and no "FAIL"
- `human_format_with_failures` -- verifies "WIRE CHECK FAILED: 2 error(s)" with file references
- `human_format_skipped_layers` -- verifies "SKIP" appears for disabled layers
- `json_format_valid` -- verifies valid JSON with correct summary counts
- `json_format_skipped` -- verifies skipped layers show status "skipped"

(Source: `src/report.rs:122-199`)

## FACTS

- Human format outputs `SKIP (not enabled)` for disabled layers, not `PASS`.
- JSON `status` field uses three values: "passed", "failed", "skipped".
- The JSON summary `passed` field is a boolean, not a count.
- Uses `serde_json::to_string_pretty` for readable JSON output.

## INFERENCES

- The SKIP/PASS distinction was added as a review finding fix (documented in `docs/specs/fix-review-findings.md`), preventing misleading output when layers are disabled.

## OPEN QUESTIONS

- None identified.

## Cross-references

- [main.rs summary](main-rs.md) -- calls `format()`
- [layers.rs summary](layers-rs.md) -- provides `Layer`, `LayerResult`, `Diagnostic` types
- [Orchestrator pattern concept](../concepts/orchestrator-pattern.md)
