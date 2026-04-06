# Concept: The Dead Code Ratchet

## What it is

A pattern where existing dead code is grandfathered but new code must be live. Dead code can only decrease over time, never increase. This makes the tool usable on large codebases where enforcing zero dead code everywhere is impractical.

## How it works in wire-check

1. Get the list of newly added `.rs` files since the git base ref (via `git diff --diff-filter=A`).
2. Run `cargo check` with `RUSTFLAGS=--force-warn dead_code` and `--message-format=json`.
3. Parse the JSON output, extracting only `dead_code` warnings.
4. Filter warnings to only those in new files.
5. Any surviving warnings = failure.

If there are zero new files, the layer passes automatically (nothing to check).

(Source: `src/layers.rs:239-265`, `src/cargo.rs:24-36`, `src/git.rs:37-72`, `README.md:63-72`)

## Inspiration

Inspired by two sources:

- **eslint-seatbelt** by justjake (Notion): Same ratchet pattern applied to ESLint rules. Grandfather existing violations, enforce on new code only.
- **The ratchet pattern** by qntm: General software development pattern where quality can only improve, never regress.

(Source: `README.md:65`)

## Why not just use cargo clippy?

`cargo clippy`'s `dead_code` lint warns about ALL dead code project-wide. On large codebases, this produces hundreds of warnings that cannot be fixed in a single PR. The ratchet approach sidesteps this: old code is untouched, new code must be clean. Over time, the codebase gets healthier without requiring a massive cleanup sprint.

(Source: `README.md:12-13`)

## Implementation details

- Uses `--force-warn` (not `-D`) so cargo completes and reports ALL warnings rather than stopping at the first.
- `main.rs` is always excluded from new file lists (it changes frequently but is not "new code" in the relevant sense).
- Test files and test modules are excluded via configurable filters.
- File matching between git output (relative paths) and cargo output (potentially different paths) uses `ends_with` in both directions.

(Source: `src/cargo.rs:28`, `src/git.rs:65`, `src/layers.rs:254-257`)

## Cross-references

- [layers.rs summary](../summaries/layers-rs.md) -- Layer 3 implementation
- [cargo.rs summary](../summaries/cargo-rs.md) -- cargo check runner
- [git.rs summary](../summaries/git-rs.md) -- new file detection
- [Four layers concept](four-layers.md) -- Layer 3 in context
- [eslint-seatbelt entity](../entities/eslint-seatbelt.md)
