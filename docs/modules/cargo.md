# cargo

## What it does

Runs `cargo check` with `--force-warn dead-code` and `--message-format=json`,
then parses the JSON output to extract dead-code warnings. Each warning
includes the file path, message text, and line number from the primary span.
Only `dead-code` warnings are returned — all other compiler messages are
filtered out.

## Key types

- `DeadCodeWarning` — file, message, line number from a dead-code warning
- `CargoError` — command failed or JSON parse failure

## Dependencies

Uses `std::process::Command` to run cargo. `serde_json` for JSON parsing.
