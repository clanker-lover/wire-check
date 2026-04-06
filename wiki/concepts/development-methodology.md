# Concept: Development Methodology

## What it is

wire-check follows a rigorous, phase-gated development methodology documented in `docs/METHODOLOGY.md` and enforced by `CLAUDE.md` rules, `clippy.toml` thresholds, and a workflow system with OS-level locks. The methodology was developed from research into how successful large Rust projects (ripgrep, Bevy, Zed, tokio, serde) manage code quality.

## Key principles

1. **Hard thresholds, not guidelines:** Size limits (500 lines/file, 40 lines/function, 5 parameters), coupling limits (5 sibling imports, 12 pub items), and complexity limits (cognitive complexity 15) are enforced mechanically via clippy.toml.

2. **Phase-gated workflow:** DESIGN -> SPEC -> IMPLEMENT -> WIRE -> DOCUMENT -> VERIFY -> NONE. Source file edits are only allowed in IMPLEMENT and WIRE phases, enforced by OS-level hooks.

3. **Machine verification over self-report:** "Claude cannot be trusted and must always be verified." Every phase gate has machine enforcement. CI runs fmt, clippy, and tests.

4. **Spec before code:** Every module starts as a spec in `docs/specs/`. The spec defines types, interfaces, errors, and test requirements before implementation begins.

5. **Documentation as architecture:** ARCHITECTURE.md documents invariants (constraints), not descriptions. It states what is ABSENT, not just what is present.

(Source: `docs/METHODOLOGY.md`, `CLAUDE.md`, `clippy.toml`, `.claude/rules/workflow.md`)

## Enforcement mechanisms

| Rule | Enforced by |
|------|------------|
| Function size/complexity | `clippy.toml` thresholds |
| No unwrap outside tests | clippy lint + code review |
| No circular dependencies | `cargo modules dependencies --acyclic` |
| Source edits only in correct phase | Workflow hook + OS locks |
| Doc coverage | doc-drift |
| Formatting | `cargo fmt --check` in CI |
| Lint cleanliness | `cargo clippy -D warnings` in CI |

(Source: `docs/METHODOLOGY.md`, `.github/workflows/ci.yml`, `doc-drift.toml`, `clippy.toml`)

## Cross-references

- [methodology-md summary](../summaries/methodology-md.md)
- [clippy.toml summary](../summaries/clippy-toml.md)
- [ci-yml summary](../summaries/ci-yml.md)
- [doc-drift.toml summary](../summaries/doc-drift-toml.md)
- [Error handling concept](error-handling.md)
