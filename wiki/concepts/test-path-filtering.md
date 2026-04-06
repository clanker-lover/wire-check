# Concept: Test Path Filtering

## What it is

A filtering system in `git.rs` that excludes test-related files from the "new files" list used by Layers 3 and 4. Test files should not be checked for dead code (Layer 3) and should not themselves require test updates (Layer 4).

## How it works

The `is_test_path()` function checks two sources:

1. **test_modules** (from config `[filters].test_modules`): Module names like `"connection_tests"`. Matches files using `/{module}.rs` and `/{module}/` path patterns. The leading slash prevents substring false positives.

2. **test_files** (from config `[filters].test_files`): Explicit file paths like `"tests/"` or `"src/connection_tests.rs"`. Matches using `starts_with` or `ends_with`.

(Source: `src/git.rs:75-89`)

## The substring matching bug

The original implementation used `.contains("test")` to identify test paths. This caused false positives: files like `src/attestation.rs` or `src/contest.rs` would be incorrectly identified as test files and skipped.

The fix (documented in `docs/specs/fix-review-findings.md`) changed to path-segment matching with `/{module}.rs` and `/{module}/` patterns. The test suite includes explicit checks for these edge cases:

- `"src/attestation.rs"` does NOT match module `"tests"`
- `"src/contest.rs"` does NOT match module `"tests"`
- `"src/latest_results.rs"` does NOT match module `"tests"`
- `"src/connection_tests.rs"` DOES match module `"connection_tests"`

(Source: `src/git.rs:186-213`, `docs/specs/fix-review-findings.md:3-25`)

## Where test_modules is also used

Besides `git.rs`, test modules are also skipped in Layer 2 (cross-reference check) -- test modules are exempt from the requirement of being referenced from outside their directory. (Source: `src/layers.rs:119-121`)

## Cross-references

- [git.rs summary](../summaries/git-rs.md)
- [layers.rs summary](../summaries/layers-rs.md)
- [Fix review findings summary](../summaries/fix-review-findings.md)
- [Four layers concept](four-layers.md)
