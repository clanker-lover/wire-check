//! Format diagnostics for human-readable or JSON output.

use crate::layers::{Layer, LayerResult};

/// Output format selection.
#[derive(Debug, Clone, Copy)]
pub(crate) enum OutputFormat {
    Human,
    Json,
}

/// Format layer results into a printable string.
pub(crate) fn format(results: &[(Layer, LayerResult)], output_format: OutputFormat) -> String {
    match output_format {
        OutputFormat::Human => format_human(results),
        OutputFormat::Json => format_json(results),
    }
}

fn format_human(results: &[(Layer, LayerResult)]) -> String {
    let mut output = String::from("=== Wire Check ===\n");
    let mut total_failures = 0usize;

    for (layer, result) in results {
        output.push_str(&format!("--- {} ---\n", layer_label(*layer)));

        match result {
            LayerResult::Skipped => {
                output.push_str("SKIP (not enabled)\n");
            }
            LayerResult::Ran(diags) => {
                if diags.is_empty() {
                    output.push_str(&format!("PASS {}\n", layer_description(*layer)));
                } else {
                    for diag in diags {
                        output.push_str(&format!(
                            "FAIL {}: {}\n",
                            diag.file.display(),
                            diag.message
                        ));
                    }
                    total_failures += diags.len();
                }
            }
        }
        output.push('\n');
    }

    output.push_str("========================\n");
    if total_failures == 0 {
        output.push_str("WIRE CHECK PASSED\n");
    } else {
        output.push_str(&format!("WIRE CHECK FAILED: {} error(s)\n", total_failures));
    }

    output
}

fn format_json(results: &[(Layer, LayerResult)]) -> String {
    let mut all_diagnostics = Vec::new();
    let mut layers_json = Vec::new();

    for (layer, result) in results {
        let label = layer_label(*layer);
        match result {
            LayerResult::Skipped => {
                layers_json.push(serde_json::json!({
                    "layer": label,
                    "status": "skipped",
                    "diagnostics": [],
                }));
            }
            LayerResult::Ran(diags) => {
                let entries: Vec<serde_json::Value> = diags
                    .iter()
                    .map(|d| {
                        serde_json::json!({
                            "file": d.file.display().to_string(),
                            "message": d.message,
                        })
                    })
                    .collect();
                layers_json.push(serde_json::json!({
                    "layer": label,
                    "status": if diags.is_empty() { "passed" } else { "failed" },
                    "diagnostics": entries,
                }));
                all_diagnostics.extend(diags.iter());
            }
        }
    }

    let result = serde_json::json!({
        "layers": layers_json,
        "summary": {
            "total": all_diagnostics.len(),
            "passed": all_diagnostics.is_empty(),
        },
    });

    serde_json::to_string_pretty(&result).unwrap_or_else(|_| "{}".to_string())
}

fn layer_label(layer: Layer) -> &'static str {
    match layer {
        Layer::AnnotationBan => "Layer 1: Annotation ban",
        Layer::CrossReference => "Layer 2: Cross-reference",
        Layer::DeadCodeRatchet => "Layer 3: Dead code ratchet",
        Layer::TestRequirement => "Layer 4: Test requirement",
    }
}

fn layer_description(layer: Layer) -> &'static str {
    match layer {
        Layer::AnnotationBan => "No dead_code suppression on module declarations",
        Layer::CrossReference => "All modules cross-referenced from outside their directories",
        Layer::DeadCodeRatchet => "No dead code in newly added files",
        Layer::TestRequirement => "Integration tests updated alongside new modules",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layers::Diagnostic;
    use std::path::PathBuf;

    fn make_diag(file: &str, message: &str) -> Diagnostic {
        Diagnostic {
            file: PathBuf::from(file),
            message: message.to_string(),
        }
    }

    #[test]
    fn human_format_all_pass() {
        let results = vec![
            (Layer::AnnotationBan, LayerResult::Ran(vec![])),
            (Layer::CrossReference, LayerResult::Ran(vec![])),
        ];
        let output = format(&results, OutputFormat::Human);
        assert!(output.contains("WIRE CHECK PASSED"));
        assert!(output.contains("PASS"));
        assert!(!output.contains("FAIL"));
    }

    #[test]
    fn human_format_with_failures() {
        let results = vec![
            (
                Layer::AnnotationBan,
                LayerResult::Ran(vec![make_diag("src/main.rs", "crate-level allow")]),
            ),
            (
                Layer::DeadCodeRatchet,
                LayerResult::Ran(vec![make_diag("src/new.rs", "function never used")]),
            ),
        ];
        let output = format(&results, OutputFormat::Human);
        assert!(output.contains("WIRE CHECK FAILED: 2 error(s)"));
        assert!(output.contains("FAIL src/main.rs"));
        assert!(output.contains("FAIL src/new.rs"));
    }

    #[test]
    fn human_format_skipped_layers() {
        let results = vec![
            (Layer::AnnotationBan, LayerResult::Ran(vec![])),
            (Layer::CrossReference, LayerResult::Skipped),
            (Layer::DeadCodeRatchet, LayerResult::Skipped),
            (Layer::TestRequirement, LayerResult::Ran(vec![])),
        ];
        let output = format(&results, OutputFormat::Human);
        assert!(output.contains("SKIP"));
        assert!(output.contains("WIRE CHECK PASSED"));
    }

    #[test]
    fn json_format_valid() {
        let results = vec![(
            Layer::CrossReference,
            LayerResult::Ran(vec![make_diag("src/main.rs", "module orphaned")]),
        )];
        let output = format(&results, OutputFormat::Json);
        let parsed: serde_json::Value = serde_json::from_str(&output).expect("valid json");
        assert_eq!(parsed["summary"]["total"], 1);
        assert_eq!(parsed["summary"]["passed"], false);
        assert_eq!(parsed["layers"][0]["status"], "failed");
    }

    #[test]
    fn json_format_skipped() {
        let results = vec![(Layer::DeadCodeRatchet, LayerResult::Skipped)];
        let output = format(&results, OutputFormat::Json);
        let parsed: serde_json::Value = serde_json::from_str(&output).expect("valid json");
        assert_eq!(parsed["layers"][0]["status"], "skipped");
        assert_eq!(parsed["summary"]["passed"], true);
    }
}
