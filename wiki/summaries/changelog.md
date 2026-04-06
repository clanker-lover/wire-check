# CHANGELOG.md -- Release History

## What it does

Records all notable changes to wire-check per release, following Keep a Changelog format and Semantic Versioning. (Source: `CHANGELOG.md`)

## Releases

### 0.1.0 (2026-03-27) -- Initial release

- Four verification layers: annotation ban, cross-reference, dead code ratchet, test requirement
- Dead code ratchet pattern: grandfathers existing dead code, enforces on new files only
- Auto-detection of crate root (main.rs/lib.rs) and git base branch (main/master)
- TOML-based per-project configuration (all optional, sensible defaults)
- Human-readable and JSON output formats
- Single-layer mode (`--layer`) for targeted checks
- Runs on stable Rust (no nightly required)

(Source: `CHANGELOG.md`)

## FACTS

- Only one release so far (0.1.0).
- Release date: 2026-03-27.
- Uses Keep a Changelog format and SemVer.

## INFERENCES

- The 0.1.0 version signals the project is in early/initial release phase.

## OPEN QUESTIONS

- None identified.

## Cross-references

- [Cargo.toml summary](cargo-toml.md) -- version field
