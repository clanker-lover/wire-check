//! The four verification layers.

use std::path::{Path, PathBuf};

use crate::cargo;
use crate::git;

/// Which layer produced a diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Layer {
    AnnotationBan,
    CrossReference,
    DeadCodeRatchet,
    TestRequirement,
}

/// A single check failure.
#[derive(Debug, Clone)]
pub(crate) struct Diagnostic {
    pub file: PathBuf,
    pub message: String,
}

/// Result of running (or skipping) a layer.
#[derive(Debug)]
pub(crate) enum LayerResult {
    /// Layer ran and produced these diagnostics (empty = pass).
    Ran(Vec<Diagnostic>),
    /// Layer was skipped (disabled or filtered out).
    Skipped,
}

/// Errors from layers that need git or cargo.
#[derive(Debug, thiserror::Error)]
pub(crate) enum LayerError {
    #[error("git error: {0}")]
    Git(#[from] git::GitError),
    #[error("cargo error: {0}")]
    Cargo(#[from] cargo::CargoError),
}

// ── Layer 1: Annotation ban ─────────────────────────────────────

/// Scan crate root for `#[allow(dead_code)]` on mod declarations
/// and crate-level `#![allow(dead_code)]`.
pub(crate) fn check_annotation_ban(crate_root: &Path) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    let content = match std::fs::read_to_string(crate_root) {
        Ok(c) => c,
        Err(_) => return diagnostics,
    };

    let lines: Vec<&str> = content.lines().collect();

    for (index, line) in lines.iter().enumerate() {
        let trimmed = line.trim();

        // Crate-level: #![allow(...dead_code...)]
        if trimmed.starts_with("#![allow(") && trimmed.contains("dead_code") {
            diagnostics.push(Diagnostic {
                file: crate_root.to_path_buf(),
                message: format!(
                    "line {}: crate-level #![allow(dead_code)] hides unwired modules",
                    index + 1
                ),
            });
        }

        // Item-level: #[allow(...dead_code...)] followed by mod declaration
        if trimmed.starts_with("#[allow(") && trimmed.contains("dead_code") {
            // Check if next non-empty line is a mod declaration.
            let next = lines.iter().skip(index + 1).find(|l| !l.trim().is_empty());

            if let Some(next_line) = next {
                let next_trimmed = next_line.trim();
                if next_trimmed.starts_with("mod ")
                    || next_trimmed.starts_with("pub mod ")
                    || next_trimmed.starts_with("pub(crate) mod ")
                {
                    diagnostics.push(Diagnostic {
                        file: crate_root.to_path_buf(),
                        message: format!(
                            "line {}: #[allow(dead_code)] on mod declaration hides unwired module",
                            index + 1
                        ),
                    });
                }
            }
        }
    }

    diagnostics
}

// ── Layer 2: Cross-reference check ──────────────────────────────

/// Every mod declared in crate root must be referenced from outside its directory.
pub(crate) fn check_cross_references(
    project_root: &Path,
    crate_root: &Path,
    test_modules: &[String],
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    let content = match std::fs::read_to_string(crate_root) {
        Ok(c) => c,
        Err(_) => return diagnostics,
    };

    let mod_names: Vec<String> = content
        .lines()
        .filter_map(|line| parse_mod_name(line.trim()))
        .collect();

    let src_dir = project_root.join("src");

    for mod_name in &mod_names {
        // Skip test modules.
        if test_modules.contains(mod_name) {
            continue;
        }

        let pattern_crate = format!("crate::{mod_name}::");
        let pattern_use = format!("use crate::{mod_name}");
        let pattern_bare = format!("{mod_name}::");

        let has_reference = find_reference_outside_module(
            &src_dir,
            mod_name,
            &[&pattern_crate, &pattern_use, &pattern_bare],
            test_modules,
        );

        if !has_reference {
            diagnostics.push(Diagnostic {
                file: crate_root.to_path_buf(),
                message: format!(
                    "module '{mod_name}' declared but never referenced from outside src/{mod_name}/"
                ),
            });
        }
    }

    diagnostics
}

/// Parse a mod name from a line like `mod foo;` or `pub(crate) mod foo;`.
fn parse_mod_name(line: &str) -> Option<String> {
    // Skip cfg(test) modules.
    if line.contains("cfg(test)") {
        return None;
    }

    // Strip pub/pub(crate) prefix.
    let after_vis = if let Some(rest) = line.strip_prefix("pub(crate) mod ") {
        rest
    } else if let Some(rest) = line.strip_prefix("pub mod ") {
        rest
    } else if let Some(rest) = line.strip_prefix("mod ") {
        rest
    } else {
        return None;
    };

    // Take the name (up to ; or {).
    let name = after_vis.trim_end_matches([';', ' ', '{']);
    if name.is_empty() || !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        return None;
    }

    Some(name.to_string())
}

/// Search src/ for any reference to the module, excluding files inside the module's own directory.
fn find_reference_outside_module(
    src_dir: &Path,
    mod_name: &str,
    patterns: &[&str],
    test_modules: &[String],
) -> bool {
    let mod_dir = src_dir.join(mod_name);
    let mod_file = src_dir.join(format!("{mod_name}.rs"));

    let mut files_to_check = Vec::new();
    collect_rs_files(src_dir, &mut files_to_check);

    for file_path in files_to_check {
        // Skip files inside the module's own directory.
        if file_path.starts_with(&mod_dir) {
            continue;
        }
        // Skip the module's own file.
        if file_path == mod_file {
            continue;
        }
        // Skip test module files.
        if test_modules.iter().any(|tm| {
            file_path.starts_with(src_dir.join(tm)) || file_path == src_dir.join(format!("{tm}.rs"))
        }) {
            continue;
        }

        let content = match std::fs::read_to_string(&file_path) {
            Ok(c) => c,
            Err(_) => continue,
        };

        for pattern in patterns {
            if content.contains(pattern) {
                return true;
            }
        }
    }

    false
}

/// Recursively collect all .rs files under a directory.
fn collect_rs_files(dir: &Path, files: &mut Vec<PathBuf>) {
    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_rs_files(&path, files);
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            files.push(path);
        }
    }
}

// ── Layer 3: Dead code ratchet ──────────────────────────────────

/// Dead code warnings filtered to only newly added files.
pub(crate) fn check_dead_code_ratchet(
    project_root: &Path,
    base_ref: &str,
    test_modules: &[String],
    test_files: &[PathBuf],
) -> Result<Vec<Diagnostic>, LayerError> {
    let new = git::new_files(project_root, base_ref, test_modules, test_files)?;
    if new.is_empty() {
        return Ok(Vec::new());
    }

    let warnings = cargo::dead_code_warnings(project_root)?;

    let diagnostics = warnings
        .iter()
        .filter(|w| {
            new.iter()
                .any(|nf| w.file.ends_with(nf) || nf.ends_with(&w.file))
        })
        .map(|w| Diagnostic {
            file: w.file.clone(),
            message: format!("line {}: {}", w.line, w.message),
        })
        .collect();

    Ok(diagnostics)
}

// ── Layer 4: Test requirement ───────────────────────────────────

/// New source files must have corresponding test file updates.
pub(crate) fn check_test_requirement(
    project_root: &Path,
    base_ref: &str,
    test_modules: &[String],
    test_files: &[PathBuf],
) -> Result<Vec<Diagnostic>, LayerError> {
    let new = git::new_files(project_root, base_ref, test_modules, test_files)?;
    if new.is_empty() {
        return Ok(Vec::new());
    }

    let modified = git::modified_files(project_root, base_ref)?;

    let test_updated = test_files
        .iter()
        .any(|tf| modified.iter().any(|mf| mf.starts_with(tf) || mf == tf));

    if test_updated {
        return Ok(Vec::new());
    }

    let file_list: Vec<String> = new.iter().map(|f| f.display().to_string()).collect();

    Ok(vec![Diagnostic {
        file: PathBuf::from("(project)"),
        message: format!(
            "new modules added ({}) but no test files updated",
            file_list.join(", ")
        ),
    }])
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── Layer 1 tests ────────────────────────────────────────

    #[test]
    fn annotation_ban_detects_crate_level() {
        let dir = tempfile::tempdir().expect("tempdir");
        let root = dir.path().join("main.rs");
        std::fs::write(&root, "#![allow(dead_code)]\nmod foo;\n").expect("write");

        let diags = check_annotation_ban(&root);
        assert_eq!(diags.len(), 1);
        assert!(diags[0].message.contains("crate-level"));
    }

    #[test]
    fn annotation_ban_detects_mod_level() {
        let dir = tempfile::tempdir().expect("tempdir");
        let root = dir.path().join("main.rs");
        std::fs::write(&root, "#[allow(dead_code)]\nmod foo;\n").expect("write");

        let diags = check_annotation_ban(&root);
        assert_eq!(diags.len(), 1);
        assert!(diags[0].message.contains("mod declaration"));
    }

    #[test]
    fn annotation_ban_passes_clean_file() {
        let dir = tempfile::tempdir().expect("tempdir");
        let root = dir.path().join("main.rs");
        std::fs::write(&root, "mod foo;\nmod bar;\n").expect("write");

        let diags = check_annotation_ban(&root);
        assert!(diags.is_empty());
    }

    #[test]
    fn annotation_ban_ignores_non_mod_allow() {
        let dir = tempfile::tempdir().expect("tempdir");
        let root = dir.path().join("main.rs");
        std::fs::write(&root, "#[allow(dead_code)]\nfn unused() {}\nmod foo;\n").expect("write");

        let diags = check_annotation_ban(&root);
        assert!(diags.is_empty());
    }

    // ── Layer 2 tests ────────────────────────────────────────

    #[test]
    fn parse_mod_name_basic() {
        assert_eq!(parse_mod_name("mod foo;"), Some("foo".to_string()));
        assert_eq!(parse_mod_name("pub mod bar;"), Some("bar".to_string()));
        assert_eq!(
            parse_mod_name("pub(crate) mod baz;"),
            Some("baz".to_string())
        );
    }

    #[test]
    fn parse_mod_name_skips_cfg_test() {
        assert_eq!(parse_mod_name("#[cfg(test)] mod tests;"), None);
    }

    #[test]
    fn parse_mod_name_rejects_non_mod() {
        assert_eq!(parse_mod_name("fn main() {}"), None);
        assert_eq!(parse_mod_name("use std::io;"), None);
    }

    #[test]
    fn cross_reference_detects_unwired_module() {
        let dir = tempfile::tempdir().expect("tempdir");
        let src = dir.path().join("src");
        std::fs::create_dir(&src).expect("mkdir");

        let crate_root = src.join("main.rs");
        std::fs::write(&crate_root, "mod orphan;\n").expect("write main");

        // Create the module file but don't reference it from anywhere.
        std::fs::write(src.join("orphan.rs"), "pub fn hello() {}\n").expect("write orphan");

        let diags = check_cross_references(dir.path(), &crate_root, &[]);
        assert_eq!(diags.len(), 1);
        assert!(diags[0].message.contains("orphan"));
    }

    #[test]
    fn cross_reference_passes_wired_module() {
        let dir = tempfile::tempdir().expect("tempdir");
        let src = dir.path().join("src");
        std::fs::create_dir(&src).expect("mkdir");

        let crate_root = src.join("main.rs");
        std::fs::write(&crate_root, "mod helper;\nuse crate::helper::greet;\n").expect("write");
        std::fs::write(src.join("helper.rs"), "pub fn greet() {}\n").expect("write");

        let diags = check_cross_references(dir.path(), &crate_root, &[]);
        assert!(diags.is_empty());
    }

    #[test]
    fn cross_reference_skips_test_modules() {
        let dir = tempfile::tempdir().expect("tempdir");
        let src = dir.path().join("src");
        std::fs::create_dir(&src).expect("mkdir");

        let crate_root = src.join("main.rs");
        std::fs::write(&crate_root, "mod tests;\n").expect("write");
        std::fs::write(src.join("tests.rs"), "").expect("write");

        let diags = check_cross_references(dir.path(), &crate_root, &["tests".to_string()]);
        assert!(diags.is_empty());
    }
}
