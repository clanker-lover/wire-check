# wire-check Wiki Index

wire-check is a dead code ratchet tool for Rust projects, published on crates.io. It runs four verification layers against a Rust project to catch code that compiles but is not connected to anything. Its key innovation: existing dead code is grandfathered, but new code must be live.

Version 0.1.0 | Repository: https://github.com/clanker-lover/wire-check

---

## Summaries

One page per significant source file. Describes what the file does, its public API, key types, dependencies, and architecture role.

### Source code

- [src/main.rs](summaries/main-rs.md) -- Binary entrypoint, CLI parsing, orchestration
- [src/cargo.rs](summaries/cargo-rs.md) -- Cargo check runner and JSON dead code parser
- [src/config.rs](summaries/config-rs.md) -- TOML configuration loading and defaults
- [src/git.rs](summaries/git-rs.md) -- Git operations (base ref, new/modified files)
- [src/layers.rs](summaries/layers-rs.md) -- The four verification layers (core logic)
- [src/report.rs](summaries/report-rs.md) -- Human and JSON output formatting

### Configuration and metadata

- [Cargo.toml](summaries/cargo-toml.md) -- Package manifest, dependencies, crates.io metadata
- [clippy.toml](summaries/clippy-toml.md) -- Clippy lint thresholds
- [doc-drift.toml](summaries/doc-drift-toml.md) -- Documentation drift verification config
- [.github/workflows/ci.yml](summaries/ci-yml.md) -- GitHub Actions CI pipeline

### Documentation

- [README.md](summaries/readme.md) -- User-facing documentation
- [CHANGELOG.md](summaries/changelog.md) -- Release history
- [docs/ARCHITECTURE.md](summaries/architecture-md.md) -- Architecture reference
- [docs/METHODOLOGY.md](summaries/methodology-md.md) -- Development rules and procedures

### Specifications

- [docs/specs/wire-check-mvp.md](summaries/mvp-spec.md) -- Original MVP specification
- [docs/specs/fix-review-findings.md](summaries/fix-review-findings.md) -- Post-review fixes
- [docs/specs/fix-cross-reference-bare-paths.md](summaries/fix-cross-reference-bare-paths.md) -- Bare path fix

---

## Concepts

Cross-cutting ideas that appear in two or more source files. Synthesize patterns across the codebase.

- [The Four Verification Layers](concepts/four-layers.md) -- What each layer catches and how
- [Dead Code Ratchet](concepts/dead-code-ratchet.md) -- The ratchet pattern: grandfather old, enforce new
- [Orchestrator Pattern](concepts/orchestrator-pattern.md) -- main.rs as pure wiring, zero business logic
- [External Command Isolation](concepts/external-command-isolation.md) -- Only git.rs and cargo.rs shell out
- [Text-Pattern Matching](concepts/text-pattern-matching.md) -- Why string search, not AST parsing
- [Test Path Filtering](concepts/test-path-filtering.md) -- How test files are excluded from checks
- [Error Handling Strategy](concepts/error-handling.md) -- Per-module thiserror, top-level eprintln+exit
- [Zero-Config Defaults](concepts/zero-config-defaults.md) -- Works with no configuration file
- [Development Methodology](concepts/development-methodology.md) -- Phase-gated workflow, hard thresholds

---

## Entities

People, tools, libraries, and external references.

- [Brandon (clanker-lover)](entities/brandon.md) -- Author and architect
- [clap](entities/clap.md) -- CLI argument parsing library
- [thiserror](entities/thiserror.md) -- Error type derivation library
- [serde / serde_json](entities/serde.md) -- Serialization framework
- [doc-drift](entities/doc-drift.md) -- Documentation verification tool (same author)
- [eslint-seatbelt](entities/eslint-seatbelt.md) -- Inspiration for the ratchet pattern
- [GitHub Actions](entities/github-actions.md) -- CI/CD platform

---

## Statistics

- **Source files:** 6 Rust files (main.rs, cargo.rs, config.rs, git.rs, layers.rs, report.rs)
- **Total tests:** 29 (across all modules)
- **Dependencies:** 6 runtime (clap, serde, serde_json, toml, thiserror, anyhow) + 1 dev (tempfile)
- **Wiki pages:** 33 total (17 summaries, 9 concepts, 7 entities)
- **Lines of Rust:** ~640 (excluding tests)

---

## Log

- [Ingest log](log.md)
