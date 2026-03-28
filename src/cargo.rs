//! Run cargo check and parse dead_code warnings from JSON output.

use std::path::{Path, PathBuf};
use std::process::Command;

/// A dead_code warning extracted from cargo check JSON output.
#[derive(Debug)]
pub(crate) struct DeadCodeWarning {
    pub file: PathBuf,
    pub message: String,
    pub line: usize,
}

/// Errors from cargo operations.
#[derive(Debug, thiserror::Error)]
pub(crate) enum CargoError {
    #[error("cargo command failed: {0}")]
    CommandFailed(String),
    #[error("failed to parse cargo output: {0}")]
    ParseFailed(String),
}

/// Run `cargo check` with `--force-warn dead_code` and parse dead_code warnings.
pub(crate) fn dead_code_warnings(project_root: &Path) -> Result<Vec<DeadCodeWarning>, CargoError> {
    let output = Command::new("cargo")
        .args(["check", "--message-format=json"])
        .env("RUSTFLAGS", "--force-warn dead_code")
        .current_dir(project_root)
        .output()
        .map_err(|e| CargoError::CommandFailed(e.to_string()))?;

    // cargo check returns non-zero when there are warnings with -D, but we're
    // using --force-warn so it should still succeed. Parse output regardless.
    let stdout = String::from_utf8_lossy(&output.stdout);
    parse_dead_code_warnings(&stdout)
}

/// Parse JSON lines from cargo check output, extracting dead_code warnings.
fn parse_dead_code_warnings(json_output: &str) -> Result<Vec<DeadCodeWarning>, CargoError> {
    let mut warnings = Vec::new();

    for line in json_output.lines() {
        if line.is_empty() {
            continue;
        }

        let value: serde_json::Value = serde_json::from_str(line)
            .map_err(|e| CargoError::ParseFailed(format!("invalid JSON line: {e}")))?;

        // Only look at compiler-message entries.
        if value.get("reason").and_then(|r| r.as_str()) != Some("compiler-message") {
            continue;
        }

        let message = match value.get("message") {
            Some(m) => m,
            None => continue,
        };

        // Check if this is a dead_code warning.
        let code = message
            .get("code")
            .and_then(|c| c.get("code"))
            .and_then(|c| c.as_str())
            .unwrap_or("");

        if code != "dead_code" {
            continue;
        }

        let msg_text = message
            .get("message")
            .and_then(|m| m.as_str())
            .unwrap_or("")
            .to_string();

        // Extract primary span file and line.
        let spans = match message.get("spans").and_then(|s| s.as_array()) {
            Some(s) => s,
            None => continue,
        };

        for span in spans {
            let is_primary = span
                .get("is_primary")
                .and_then(|p| p.as_bool())
                .unwrap_or(false);

            if !is_primary {
                continue;
            }

            let file = span
                .get("file_name")
                .and_then(|f| f.as_str())
                .unwrap_or("")
                .to_string();

            let line = span.get("line_start").and_then(|l| l.as_u64()).unwrap_or(0) as usize;

            if !file.is_empty() {
                warnings.push(DeadCodeWarning {
                    file: PathBuf::from(file),
                    message: msg_text.clone(),
                    line,
                });
            }

            break;
        }
    }

    Ok(warnings)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_JSON: &str = r#"{"reason":"compiler-artifact","package_id":"foo","target":{"name":"foo"},"filenames":[]}
{"reason":"compiler-message","package_id":"foo","message":{"code":{"code":"dead_code","explanation":null},"level":"warning","message":"function `unused_helper` is never used","spans":[{"file_name":"src/helpers.rs","byte_start":100,"byte_end":120,"line_start":5,"line_end":5,"column_start":1,"column_end":20,"is_primary":true}]}}
{"reason":"compiler-message","package_id":"foo","message":{"code":{"code":"unused_imports","explanation":null},"level":"warning","message":"unused import: `std::io`","spans":[{"file_name":"src/main.rs","byte_start":0,"byte_end":10,"line_start":1,"line_end":1,"column_start":1,"column_end":10,"is_primary":true}]}}
{"reason":"build-finished","success":true}"#;

    #[test]
    fn parse_extracts_dead_code_only() {
        let warnings = parse_dead_code_warnings(SAMPLE_JSON).expect("parse");
        assert_eq!(warnings.len(), 1);
        assert_eq!(warnings[0].file, PathBuf::from("src/helpers.rs"));
        assert!(warnings[0].message.contains("unused_helper"));
        assert_eq!(warnings[0].line, 5);
    }

    #[test]
    fn parse_empty_input() {
        let warnings = parse_dead_code_warnings("").expect("parse");
        assert!(warnings.is_empty());
    }

    #[test]
    fn parse_no_dead_code() {
        let json = r#"{"reason":"build-finished","success":true}"#;
        let warnings = parse_dead_code_warnings(json).expect("parse");
        assert!(warnings.is_empty());
    }
}
