# .github/workflows/ci.yml -- CI Pipeline

## What it does

GitHub Actions workflow that runs on push to master and pull requests targeting master. Performs three checks: formatting, linting, and testing. (Source: `.github/workflows/ci.yml`)

## Pipeline steps

1. Checkout code (`actions/checkout@v4`)
2. Install stable Rust toolchain (`dtolnay/rust-toolchain@stable`)
3. Cache Rust artifacts (`Swatinem/rust-cache@v2`)
4. `cargo fmt --check` -- formatting verification
5. `cargo clippy --all-targets -- -D warnings` -- lint with warnings-as-errors
6. `cargo test` -- run all tests

(Source: `.github/workflows/ci.yml`)

## FACTS

- Runs on `ubuntu-latest`.
- Uses `CARGO_TERM_COLOR: always` for colored output.
- Does NOT run wire-check on itself (no `wire-check .` step).
- Does NOT run `cargo doc` or acyclic dependency checks in CI.
- Single job named "Check, lint, test".

## INFERENCES

- The CI is minimal -- just fmt, clippy, test. The more extensive gauntlet described in `docs/METHODOLOGY.md` (doc coverage, acyclic checks, mutation testing) is for local development, not CI.

## OPEN QUESTIONS

- Should CI run wire-check on itself as a dogfooding step?

## Cross-references

- [Methodology concept](../concepts/development-methodology.md)
- [GitHub Actions entity](../entities/github-actions.md)
