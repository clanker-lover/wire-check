# docs/ARCHITECTURE.md -- Architecture Document

## What it does

Master architecture reference for the project. Contains the bird's eye view, codemap (every module with one-paragraph description), architectural invariants, data flow diagram, and cross-cutting concerns. (Source: `docs/ARCHITECTURE.md`)

## Key sections

**Codemap:** Six entries covering all source files with brief descriptions. (Source: `docs/ARCHITECTURE.md:12-47`)

**Architectural invariants (4):**
1. `main.rs` contains no business logic -- only wiring and I/O.
2. No module runs external commands except `git.rs` and `cargo.rs`.
3. All layers return `Vec<Diagnostic>` -- no direct printing.
4. No module has global mutable state.

(Source: `docs/ARCHITECTURE.md:50-60`)

**Hard thresholds:**
- Each module does ONE thing (one sentence)
- Modules communicate through function calls with explicit types
- No module imports more than 5 siblings
- No module exposes more than 12 pub items
- No file exceeds 500 logic lines (excluding tests)

(Source: `docs/ARCHITECTURE.md:62-67`)

**Data flow:** CLI args + config -> main.rs -> four layers in parallel -> Vec<Diagnostic> -> report.rs -> stdout. (Source: `docs/ARCHITECTURE.md:69-85`)

**Cross-cutting concerns:** Error handling (thiserror per module, eprintln+exit(2) in main), testing (29 unit tests, in-module), visibility (pub(crate) default). (Source: `docs/ARCHITECTURE.md:87-101`)

## FACTS

- 29 tests total across all modules.
- The data flow diagram shows layers conceptually in parallel, though code executes them sequentially.

## INFERENCES

- The invariants are written as constraints ("violations are bugs"), not descriptions. This is intentional per the methodology.

## OPEN QUESTIONS

- None identified.

## Cross-references

- [Orchestrator pattern concept](../concepts/orchestrator-pattern.md)
- [External command isolation concept](../concepts/external-command-isolation.md)
- [Development methodology concept](../concepts/development-methodology.md)
