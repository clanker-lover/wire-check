# docs/specs/fix-review-findings.md -- Post-Review Fix Spec

## What it does

Specification for six fixes identified during code review of the initial implementation. Covers a bug fix, a reporting improvement, a cleanup, and several polish items. (Source: `docs/specs/fix-review-findings.md`)

## Fixes specified

1. **git.rs test file filtering bug:** Replace `.contains("test")` substring match with proper path-based filtering using `/{module}.rs` and `/{module}/` patterns. Prevents false positives like "attestation.rs" matching "tests". (Source: `docs/specs/fix-review-findings.md:3-25`)

2. **SKIP vs PASS reporting:** Add `LayerResult` enum with `Ran(Vec<Diagnostic>)` and `Skipped` variants. Disabled layers now show "SKIP" instead of misleading "PASS". (Source: `docs/specs/fix-review-findings.md:27-39`)

3. **Delete stub ADR:** Remove placeholder `docs/decisions/0001-initial-architecture.md`. (Source: `docs/specs/fix-review-findings.md:41-42`)

4. **Edition and rust-version:** Set `edition = "2021"` and `rust-version` to match clap 4's MSRV. (Source: `docs/specs/fix-review-findings.md:44-47`)

5. **Remove exclude_paths:** Delete unused `exclude_paths` field from `FiltersConfig` and its `#[allow(dead_code)]`. (Source: `docs/specs/fix-review-findings.md:49-51`)

6. **README Known Limitations:** Add section documenting text-pattern-based cross-reference check limitations. (Source: `docs/specs/fix-review-findings.md:53-54`)

## FACTS

- All six fixes have been implemented in the current codebase.
- The test file filtering bug (#1) is the most architecturally significant fix.
- The SKIP vs PASS change (#2) required adding a new enum variant to the type system.

## INFERENCES

- The review process caught real bugs (substring matching) and UX issues (misleading PASS for disabled layers), validating the spec-then-implement workflow.

## OPEN QUESTIONS

- None identified.

## Cross-references

- [git.rs summary](git-rs.md) -- fix #1
- [report.rs summary](report-rs.md) -- fix #2
- [layers.rs summary](layers-rs.md) -- LayerResult enum
- [Test path filtering concept](../concepts/test-path-filtering.md)
