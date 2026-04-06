# Entity: eslint-seatbelt

## What it is

A JavaScript/TypeScript tool by justjake (Notion engineer) that implements the ratchet pattern for ESLint rules. Repository: https://github.com/justjake/eslint-seatbelt

## Relationship to wire-check

wire-check's dead code ratchet (Layer 3) is directly inspired by eslint-seatbelt's approach: grandfather existing violations, enforce on new code only. The README credits both eslint-seatbelt and qntm's ratchet pattern article as the conceptual foundation.

(Source: `README.md:65`)

## The ratchet pattern

The general concept (described by qntm at https://qntm.org/ratchet) is a software development pattern where quality can only improve, never regress. Applied to linting: existing violations are allowed to remain, but adding new violations is blocked. Over time, as old code is touched and cleaned up, the violation count can only decrease.

(Source: `README.md:65-72`)

## Cross-references

- [Dead code ratchet concept](../concepts/dead-code-ratchet.md)
- [README summary](../summaries/readme.md)
