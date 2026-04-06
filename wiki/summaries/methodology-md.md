# docs/METHODOLOGY.md -- Development Rules and Procedures

## What it does

Comprehensive methodology document defining 41 hard rules (not guidelines) for Rust project development, plus procedures for new modules, growing projects, and cluster audits. Sourced from deep research into ripgrep, Bevy, Zed, tokio, and serde codebases. (Source: `docs/METHODOLOGY.md`)

## Key sections

**Size rules:** 500 line max per file, 12 pub items max, 40 line max per function, 5 parameter max, complexity < 15. (Source: `docs/METHODOLOGY.md:10-17`)

**Coupling rules:** No circular deps (cargo modules --acyclic), max 5 sibling imports, pub(crate) default, no wildcards, no global mutable state. (Source: `docs/METHODOLOGY.md:19-25`)

**Testing rules:** Every public function tested, 80% coverage minimum, no unwrap in library code, thiserror for errors, monthly mutation testing. (Source: `docs/METHODOLOGY.md:33-41`)

**Integration testing rules (21-26):** Pairwise connection tests, full pipeline test, timeout on recv, start_paused for time tests, every module wired into main.rs, modules accept channel halves. (Source: `docs/METHODOLOGY.md:42-47`)

**Scaling rules:** Start single crate, split at 8K lines or 60s compile. Flat crates/ layout. (Source: `docs/METHODOLOGY.md:53-56`)

**Documentation rules:** ARCHITECTURE.md required, ADRs in docs/decisions/, document what is ABSENT. (Source: `docs/METHODOLOGY.md:58-68`)

**AI-specific rules:** Never generate more than one module at a time, full gauntlet after AI generation, anti-pattern scan. (Source: `docs/METHODOLOGY.md:82-85`)

**Trust rule:** Claude cannot be trusted, must always be machine-verified. (Source: `docs/METHODOLOGY.md:88-94`)

**Health checks:** Before every commit (fmt, clippy, test, acyclic), after every new module, weekly (coverage, orphans, machete), monthly (mutants, outdated, geiger). (Source: `docs/METHODOLOGY.md:135-167`)

## FACTS

- 41 numbered rules total.
- References real codebases (ripgrep, Bevy, Zed, tokio, serde) as sources.
- Includes explicit adversarial review procedure for AI-generated code.
- Recommends crabviz VS Code extension for call graphs.

## INFERENCES

- This methodology was developed for a broader context (multi-module Rust projects) and applied to wire-check. Many rules (workspace splits, channel-based testing, tokio) do not apply to wire-check's current scope.

## OPEN QUESTIONS

- None identified.

## Cross-references

- [Development methodology concept](../concepts/development-methodology.md)
- [clippy.toml summary](clippy-toml.md) -- enforces thresholds
