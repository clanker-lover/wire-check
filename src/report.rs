//! Format diagnostics for human-readable or JSON output.

use crate::layers::{Diagnostic, Layer};

/// Output format selection.
#[derive(Debug, Clone, Copy)]
pub(crate) enum OutputFormat {
    Human,
    Json,
}

/// Format diagnostics into a printable string.
pub(crate) fn format(diagnostics: &[Diagnostic], output_format: OutputFormat) -> String {
    match output_format {
        OutputFormat::Human => format_human(diagnostics),
        OutputFormat::Json => format_json(diagnostics),
    }
}

fn format_human(diagnostics: &[Diagnostic]) -> String {
    let mut output = String::from("=== Wire Check ===\n");

    // Group by layer for readable output.
    for layer in &[
        Layer::AnnotationBan,
        Layer::CrossReference,
        Layer::DeadCodeRatchet,
        Layer::TestRequirement,
    ] {
        let layer_diags: Vec<&Diagnostic> =
            diagnostics.iter().filter(|d| d.layer == *layer).collect();

        output.push_str(&format!("--- {} ---\n", layer_label(*layer)));

        if layer_diags.is_empty() {
            output.push_str(&format!("PASS {}\n", layer_description(*layer)));
        } else {
            for diag in &layer_diags {
                output.push_str(&format!("FAIL {}: {}\n", diag.file.display(), diag.message));
            }
        }
        output.push('\n');
    }

    output.push_str("========================\n");
    if diagnostics.is_empty() {
        output.push_str("WIRE CHECK PASSED\n");
    } else {
        output.push_str(&format!(
            "WIRE CHECK FAILED: {} error(s)\n",
            diagnostics.len()
        ));
    }

    output
}

fn format_json(diagnostics: &[Diagnostic]) -> String {
    let entries: Vec<serde_json::Value> = diagnostics
        .iter()
        .map(|d| {
            serde_json::json!({
                "layer": layer_label(d.layer),
                "file": d.file.display().to_string(),
                "message": d.message,
            })
        })
        .collect();

    let result = serde_json::json!({
        "diagnostics": entries,
        "summary": {
            "total": diagnostics.len(),
            "passed": diagnostics.is_empty(),
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
    use std::path::PathBuf;

    fn make_diag(layer: Layer, file: &str, message: &str) -> Diagnostic {
        Diagnostic {
            layer,
            file: PathBuf::from(file),
            message: message.to_string(),
        }
    }

    #[test]
    fn human_format_all_pass() {
        let output = format(&[], OutputFormat::Human);
        assert!(output.contains("WIRE CHECK PASSED"));
        assert!(output.contains("PASS"));
        assert!(!output.contains("FAIL"));
    }

    #[test]
    fn human_format_with_failures() {
        let diags = vec![
            make_diag(Layer::AnnotationBan, "src/main.rs", "crate-level allow"),
            make_diag(Layer::DeadCodeRatchet, "src/new.rs", "function never used"),
        ];
        let output = format(&diags, OutputFormat::Human);
        assert!(output.contains("WIRE CHECK FAILED: 2 error(s)"));
        assert!(output.contains("FAIL src/main.rs"));
        assert!(output.contains("FAIL src/new.rs"));
    }

    #[test]
    fn json_format_valid() {
        let diags = vec![make_diag(
            Layer::CrossReference,
            "src/main.rs",
            "module orphaned",
        )];
        let output = format(&diags, OutputFormat::Json);
        let parsed: serde_json::Value = serde_json::from_str(&output).expect("valid json");
        assert_eq!(parsed["summary"]["total"], 1);
        assert_eq!(parsed["summary"]["passed"], false);
    }

    #[test]
    fn json_format_empty() {
        let output = format(&[], OutputFormat::Json);
        let parsed: serde_json::Value = serde_json::from_str(&output).expect("valid json");
        assert_eq!(parsed["summary"]["total"], 0);
        assert_eq!(parsed["summary"]["passed"], true);
    }
}
