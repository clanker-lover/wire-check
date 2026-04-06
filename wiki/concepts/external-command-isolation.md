# Concept: External Command Isolation

## What it is

An architectural invariant: only two modules are allowed to run external commands. `git.rs` runs `git`, and `cargo.rs` runs `cargo check`. All other modules operate on in-memory data only.

(Source: `docs/ARCHITECTURE.md:53-55`)

## Why it matters

- **Testability:** Modules that don't shell out can be tested with pure in-memory data (strings, paths, structs). Only `git.rs` and `cargo.rs` need real filesystems and git repos for testing.
- **Predictability:** Side effects are confined to two known locations. If a test hangs or fails due to environment issues, you know where to look.
- **Portability:** The business logic (layers, config parsing, report formatting) works identically regardless of the git or cargo binary version.

## How it's enforced

Through code review and the architectural invariant in `docs/ARCHITECTURE.md`. There is no automated enforcement (no lint rule prevents `Command::new` in other modules).

## Modules and their I/O classification

| Module | External commands | File reads | Pure logic |
|--------|------------------|-----------|------------|
| `cargo.rs` | `cargo check` | -- | JSON parsing |
| `git.rs` | `git rev-parse`, `git diff` | -- | path filtering |
| `config.rs` | -- | `wire-check.toml`, `src/main.rs`, `src/lib.rs` | TOML parsing |
| `layers.rs` | -- | crate root, `src/**/*.rs` | all four checks |
| `report.rs` | -- | -- | formatting |
| `main.rs` | -- | -- | orchestration |

(Source: `src/cargo.rs`, `src/git.rs`, `src/config.rs`, `src/layers.rs`, `src/report.rs`, `src/main.rs`)

## Cross-references

- [cargo.rs summary](../summaries/cargo-rs.md)
- [git.rs summary](../summaries/git-rs.md)
- [architecture-md summary](../summaries/architecture-md.md)
- [Orchestrator pattern concept](orchestrator-pattern.md)
