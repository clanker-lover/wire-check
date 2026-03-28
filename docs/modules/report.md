# report

## What it does

Formats diagnostics for human-readable or JSON output. Human format shows
each layer with PASS/FAIL status, grouping diagnostics under their layer.
JSON format produces a structured object with a diagnostics array and
summary. The module never decides what to check — it only formats what
the layers found.

## Key types

- `OutputFormat` — Human or Json

## Dependencies

Uses `serde_json` for JSON output. Imports `Diagnostic` and `Layer` from
the layers module.
