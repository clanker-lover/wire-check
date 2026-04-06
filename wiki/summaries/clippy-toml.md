# clippy.toml -- Clippy Configuration

## What it does

Sets project-level thresholds for Rust Clippy lints, enforcing code complexity and size limits. (Source: `clippy.toml`)

## Thresholds

| Setting | Value | Meaning |
|---------|-------|---------|
| `cognitive-complexity-threshold` | 15 | Max nesting/branching complexity per function |
| `too-many-arguments-threshold` | 5 | Max function parameters (use struct instead) |
| `too-many-lines-threshold` | 40 | Max logic lines per function |
| `type-complexity-threshold` | 250 | Max type signature complexity (use alias) |

(Source: `clippy.toml`)

## FACTS

- These thresholds match the values documented in `docs/METHODOLOGY.md`.
- The 40-line and 5-parameter limits are described as "hard thresholds, not guidelines" in the methodology.

## INFERENCES

- These limits enforce the methodology rules mechanically rather than relying on manual review.

## OPEN QUESTIONS

- None identified.

## Cross-references

- [Development methodology concept](../concepts/development-methodology.md)
