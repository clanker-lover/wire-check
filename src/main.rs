//! wire-check — verify newly added Rust code is actually integrated.

mod cargo;
mod config;
mod git;
mod layers;
mod report;

use std::path::PathBuf;
use std::process;

use clap::Parser;

/// Verify newly added Rust code is actually integrated into the project.
#[derive(Parser)]
#[command(version, about)]
struct Cli {
    /// Path to the Rust project (default: current directory).
    #[arg(default_value = ".")]
    project_root: PathBuf,

    /// Config file path (default: <PROJECT_ROOT>/wire-check.toml).
    #[arg(short, long)]
    config: Option<PathBuf>,

    /// Output format: human, json.
    #[arg(short, long, default_value = "human")]
    format: String,

    /// Override git base ref (default: auto-detect main/master).
    #[arg(long)]
    base_ref: Option<String>,

    /// Run only a specific layer: annotation-ban, cross-reference, dead-code-ratchet, test-requirement.
    #[arg(long)]
    layer: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    let project_root = match cli.project_root.canonicalize() {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "Error: invalid project path '{}': {e}",
                cli.project_root.display()
            );
            process::exit(2);
        }
    };

    let output_format = match cli.format.as_str() {
        "human" => report::OutputFormat::Human,
        "json" => report::OutputFormat::Json,
        other => {
            eprintln!("Error: unknown format '{other}'. Use 'human' or 'json'.");
            process::exit(2);
        }
    };

    // Load config.
    let config_path = cli
        .config
        .unwrap_or_else(|| project_root.join("wire-check.toml"));

    let project_config = if config_path.exists() {
        match config::load(&config_path) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Error loading config: {e}");
                process::exit(2);
            }
        }
    } else {
        config::defaults()
    };

    // Resolve crate root.
    let crate_root = project_config
        .project
        .crate_root
        .as_ref()
        .map(|p| project_root.join(p))
        .or_else(|| config::detect_crate_root(&project_root));

    let crate_root = match crate_root {
        Some(p) => p,
        None => {
            eprintln!("Error: could not find src/main.rs or src/lib.rs");
            process::exit(2);
        }
    };

    // Resolve base ref.
    let base_ref = if let Some(ref cli_ref) = cli.base_ref {
        cli_ref.clone()
    } else if project_config.project.base_ref == "auto" {
        match git::detect_base_ref(&project_root) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("Error detecting base ref: {e}");
                process::exit(2);
            }
        }
    } else {
        project_config.project.base_ref.clone()
    };

    // Determine which layers to run.
    let run_layer = |name: &str, enabled: bool| -> bool {
        match &cli.layer {
            Some(filter) => filter == name,
            None => enabled,
        }
    };

    // Run layers, collect diagnostics.
    let mut diagnostics = Vec::new();

    if run_layer("annotation-ban", project_config.layers.annotation_ban) {
        diagnostics.extend(layers::check_annotation_ban(&crate_root));
    }

    if run_layer("cross-reference", project_config.layers.cross_reference) {
        diagnostics.extend(layers::check_cross_references(
            &project_root,
            &crate_root,
            &project_config.filters.test_modules,
        ));
    }

    if run_layer("dead-code-ratchet", project_config.layers.dead_code_ratchet) {
        match layers::check_dead_code_ratchet(&project_root, &base_ref) {
            Ok(diags) => diagnostics.extend(diags),
            Err(e) => {
                eprintln!("Error in dead code ratchet: {e}");
                process::exit(2);
            }
        }
    }

    if run_layer("test-requirement", project_config.layers.test_requirement) {
        match layers::check_test_requirement(
            &project_root,
            &base_ref,
            &project_config.filters.test_files,
        ) {
            Ok(diags) => diagnostics.extend(diags),
            Err(e) => {
                eprintln!("Error in test requirement check: {e}");
                process::exit(2);
            }
        }
    }

    // Report.
    let output = report::format(&diagnostics, output_format);
    print!("{output}");

    if diagnostics.is_empty() {
        process::exit(0);
    } else {
        process::exit(1);
    }
}
