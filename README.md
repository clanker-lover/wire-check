# wire-check

[![Crates.io](https://img.shields.io/crates/v/wire-check.svg?style=flat-square)](https://crates.io/crates/wire-check)
[![License](https://img.shields.io/crates/l/wire-check.svg?style=flat-square)](LICENSE-MIT)

Verify newly added Rust code is actually integrated — not just compiled, but used.

## What It Does

wire-check runs four verification layers against your Rust project, catching code that compiles but isn't connected to anything. It's a CI gate that prevents dead modules from accumulating.

No existing tool does this. `cargo clippy` warns about dead code project-wide (unusable on large codebases), and `cargo-modules orphans` only finds files missing from the module tree. wire-check's key innovation is the **dead code ratchet**: existing dead code is grandfathered, but new code must be live. Dead code can only decrease over time, never increase.

## Why

Every growing Rust project accumulates modules that compile but aren't wired into anything — built for a future integration, left behind after a refactor, or merged without connection tests. These modules pass `cargo check` and `cargo test` but do nothing. wire-check catches them at the PR stage.

## Installation

```bash
cargo install wire-check
```

## Quick Start

Run it in any Rust project:

```bash
wire-check
```

Output:

```
=== Wire Check ===
--- Layer 1: Annotation ban ---
PASS No dead_code suppression on module declarations

--- Layer 2: Cross-reference ---
FAIL src/main.rs: module 'orphan' declared but never referenced from outside src/orphan/

--- Layer 3: Dead code ratchet ---
FAIL src/research/mod.rs: line 42: function `unused_helper` is never used

--- Layer 4: Test requirement ---
PASS Integration tests updated alongside new modules

========================
WIRE CHECK FAILED: 2 error(s)
```

Exit code 0 if all checks pass, 1 if any fail, 2 on errors.

## Four Layers

| Layer | What It Catches |
|-------|----------------|
| **Annotation ban** | `#[allow(dead_code)]` on mod declarations in the crate root — hides unwired modules from the compiler |
| **Cross-reference** | Modules declared in the crate root but never imported from outside their own directory |
| **Dead code ratchet** | Dead code warnings in *newly added files only* — existing dead code is grandfathered, new code must be live |
| **Test requirement** | New source files added without corresponding test file updates |

## The Ratchet Pattern

The dead code ratchet is inspired by [Notion's eslint-seatbelt](https://github.com/justjake/eslint-seatbelt) and the [ratchet pattern in software development](https://qntm.org/ratchet). Instead of enforcing zero dead code everywhere (impossible on large codebases), it enforces that dead code *only decreases over time*:

1. Get the list of newly added `.rs` files since the git base ref
2. Run `cargo check` with `--force-warn dead_code`
3. Filter warnings to only those in new files
4. Any surviving warnings = failure

Old code is untouched. New code must be clean. Over time, the codebase gets healthier without requiring a massive cleanup sprint.

## Configuration

Create a `wire-check.toml` in your project root (all fields optional):

```toml
[project]
# Crate root file (default: auto-detect src/main.rs or src/lib.rs)
crate_root = "src/main.rs"
# Git base ref (default: "auto" — detects main/master)
base_ref = "auto"

[layers]
# Enable/disable individual layers (all default to true)
annotation_ban = true
cross_reference = true
dead_code_ratchet = true
test_requirement = true

[filters]
# Module names exempt from cross-reference check (e.g., test-only modules)
test_modules = ["connection_tests", "functional_tests"]
# Files that count as "test files" for the test requirement layer
test_files = ["src/connection_tests.rs", "tests/"]
```

No config file needed for basic usage — all defaults apply.

## CLI Options

```
wire-check [OPTIONS] [PROJECT_ROOT]

Arguments:
  [PROJECT_ROOT]  Path to the Rust project (default: current directory)

Options:
  -c, --config <FILE>     Config file path (default: <PROJECT_ROOT>/wire-check.toml)
  -f, --format <FMT>      Output format: human, json (default: human)
      --base-ref <REF>     Override git base ref
      --layer <LAYER>      Run only a specific layer:
                           annotation-ban, cross-reference, dead-code-ratchet, test-requirement
  -h, --help               Print help
  -V, --version            Print version
```

Use `--format json` for CI integration:

```bash
wire-check --format json | jq '.summary'
```

## CI Integration

### GitHub Actions

```yaml
- name: Wire check
  run: |
    cargo install wire-check
    wire-check .
```

### As a Workflow Gate

wire-check was designed to run at integration points in a development workflow — after code is written but before documentation and final verification. It answers: "is this new code actually connected to the rest of the system?"

## Requirements

- Stable Rust toolchain (no nightly required)
- A git repository with at least one commit
- A Cargo project with a `Cargo.toml`

## Known Limitations

- **Cross-reference check is text-pattern-based.** It searches for `crate::module::` and `module::` strings in source files rather than parsing the AST. This means it could produce false positives from references in comments or string literals, or miss references through re-exports. This is sufficient for typical Rust projects and keeps the tool dependency-free (no syn, no rust-analyzer), but it's worth knowing.
- **Dead code ratchet requires git history.** The tool needs at least one commit to compute diffs against. It won't work on a freshly initialized repo with no commits.
- **Single-crate projects only.** Workspace support (multiple crates with cross-crate references) is not yet implemented.

## Related Tools

| Tool | What It Does | How wire-check Differs |
|------|-------------|----------------------|
| `cargo clippy` (dead_code) | Warns about all dead code project-wide | wire-check only checks *new* files — the ratchet |
| `cargo-modules orphans` | Finds .rs files not in the module tree | wire-check finds modules *declared* but never *used* |
| `cargo-shear` | Finds unused Cargo.toml dependencies | Different scope — dependency hygiene vs code integration |
| `cargo-udeps` | Finds unused dependencies (requires nightly) | Different scope |

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
