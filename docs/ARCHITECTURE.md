# Architecture

## Bird's Eye View

wire-check verifies that newly added Rust source code is actually integrated
into a project — not just compiled but referenced and tested. It runs as a CI
gate or development workflow check. Takes a project directory, runs four
verification layers, reports diagnostics. Single-crate CLI tool.

## Codemap

### `src/main.rs`
Binary entrypoint. Parses CLI args (clap), loads config, resolves crate root
and git base ref, runs enabled layers, formats output, exits with appropriate
code (0=pass, 1=fail, 2=error). Contains no business logic.

### `src/config.rs`
TOML config loading and defaults. Defines `Config`, `ProjectConfig`,
`LayersConfig`, `FiltersConfig`. Auto-detects crate root (main.rs or lib.rs).
Missing config file = all defaults (all layers enabled, auto-detect everything).

### `src/git.rs`
Git operations via `std::process::Command`. Detects base branch (main/master),
lists newly added files (`--diff-filter=A`), lists all modified files. All
paths relative to project root.

### `src/cargo.rs`
Runs `cargo check` with `--force-warn dead_code` and `--message-format=json`.
Parses JSON output line by line, extracts `dead_code` warnings with file path,
message, and line number.

### `src/layers.rs`
The four verification checks:
- **Layer 1 (annotation_ban):** Scans crate root for `#[allow(dead_code)]` on
  mod declarations and crate-level `#![allow(dead_code)]`.
- **Layer 2 (cross_reference):** Verifies every `mod` in crate root is imported
  from outside its own directory. Skips configured test modules.
- **Layer 3 (dead_code_ratchet):** Filters cargo dead_code warnings to only
  newly added files. Existing dead code is grandfathered.
- **Layer 4 (test_requirement):** Checks that new source files have
  corresponding test file updates.

### `src/report.rs`
Formats diagnostics for human-readable or JSON output. Human format groups by
layer with PASS/FAIL per layer. JSON format is an array of diagnostics with
summary.

## Architectural Invariants

**Architecture Invariant:** `main.rs` contains no business logic. It only wires
modules together and handles I/O.

**Architecture Invariant:** No module runs external commands except `git.rs`
(runs `git`) and `cargo.rs` (runs `cargo check`). All other modules operate
on in-memory data only.

**Architecture Invariant:** All layers return `Vec<Diagnostic>`. Layers never
print output directly. Formatting is report.rs's responsibility.

**Architecture Invariant:** No module has global mutable state.

### Hard Thresholds
- Each module does ONE thing (describable in one sentence)
- Modules communicate through function calls with explicit types
- No module imports more than 5 siblings
- No module exposes more than 12 pub items
- No file exceeds 500 logic lines (excluding tests)

## Data Flow
```
CLI args + wire-check.toml
         │
    main.rs (resolve config, crate root, base ref)
         │
    ┌────┼────┬──────────┐
    │    │    │          │
 Layer1 Layer2 Layer3  Layer4
 (scan)  (scan) (git+cargo) (git)
    │    │    │          │
    └────┼────┴──────────┘
         │
  Vec<Diagnostic>
         │
    report.rs → stdout
```

## Cross-Cutting Concerns

### Error Handling
Each module defines its own error type using `thiserror`.
`main.rs` prints errors via `eprintln!` and exits with code 2.
No `unwrap()` or `expect()` outside test code.

### Testing
Unit tests inside each module (`#[cfg(test)] mod tests`).
29 tests covering all layers, config loading, git operations, cargo JSON
parsing, and output formatting.

### Visibility
Default to `pub(crate)`, not `pub`. Only external API gets `pub`.
No wildcard imports.
