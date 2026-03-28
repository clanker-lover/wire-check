//! Configuration loading for wire-check.toml.

use std::path::{Path, PathBuf};

use serde::Deserialize;

/// Top-level configuration.
#[derive(Debug, Clone, Deserialize)]
pub(crate) struct Config {
    #[serde(default)]
    pub project: ProjectConfig,
    #[serde(default)]
    pub layers: LayersConfig,
    #[serde(default)]
    pub filters: FiltersConfig,
}

/// Project-level settings.
#[derive(Debug, Clone, Deserialize)]
pub(crate) struct ProjectConfig {
    /// Crate root file. None = auto-detect src/main.rs or src/lib.rs.
    #[serde(default)]
    pub crate_root: Option<PathBuf>,
    /// Git base ref. "auto" = detect main/master.
    #[serde(default = "default_base_ref")]
    pub base_ref: String,
}

/// Which layers to run.
#[derive(Debug, Clone, Deserialize)]
pub(crate) struct LayersConfig {
    #[serde(default = "default_true")]
    pub annotation_ban: bool,
    #[serde(default = "default_true")]
    pub cross_reference: bool,
    #[serde(default = "default_true")]
    pub dead_code_ratchet: bool,
    #[serde(default = "default_true")]
    pub test_requirement: bool,
}

/// Paths and module names to exclude from checks.
#[derive(Debug, Clone, Default, Deserialize)]
pub(crate) struct FiltersConfig {
    /// Module names exempt from cross-reference check.
    #[serde(default)]
    pub test_modules: Vec<String>,
    /// Files that count as "test files" for Layer 4.
    #[serde(default)]
    pub test_files: Vec<PathBuf>,
}

fn default_base_ref() -> String {
    "auto".to_string()
}

fn default_true() -> bool {
    true
}

impl Default for ProjectConfig {
    fn default() -> Self {
        Self {
            crate_root: None,
            base_ref: default_base_ref(),
        }
    }
}

impl Default for LayersConfig {
    fn default() -> Self {
        Self {
            annotation_ban: true,
            cross_reference: true,
            dead_code_ratchet: true,
            test_requirement: true,
        }
    }
}

/// Errors from config loading.
#[derive(Debug, thiserror::Error)]
pub(crate) enum ConfigError {
    #[error("config file not found: {0}")]
    NotFound(PathBuf),
    #[error("failed to read config: {0}")]
    Read(#[from] std::io::Error),
    #[error("invalid config: {0}")]
    Parse(#[from] toml::de::Error),
}

/// Load configuration from a TOML file.
pub(crate) fn load(path: &Path) -> Result<Config, ConfigError> {
    if !path.exists() {
        return Err(ConfigError::NotFound(path.to_path_buf()));
    }
    let contents = std::fs::read_to_string(path)?;
    let config: Config = toml::from_str(&contents)?;
    Ok(config)
}

/// Return default config when no file exists.
pub(crate) fn defaults() -> Config {
    Config {
        project: ProjectConfig::default(),
        layers: LayersConfig::default(),
        filters: FiltersConfig::default(),
    }
}

/// Auto-detect crate root: src/main.rs first, then src/lib.rs.
pub(crate) fn detect_crate_root(project_root: &Path) -> Option<PathBuf> {
    let main = project_root.join("src/main.rs");
    if main.exists() {
        return Some(main);
    }
    let lib = project_root.join("src/lib.rs");
    if lib.exists() {
        return Some(lib);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_valid_config() {
        let dir = tempfile::tempdir().expect("tempdir");
        let config_path = dir.path().join("wire-check.toml");
        std::fs::write(
            &config_path,
            r#"
[project]
crate_root = "src/main.rs"
base_ref = "main"

[layers]
annotation_ban = true
cross_reference = false

[filters]
test_modules = ["tests"]
"#,
        )
        .expect("write");

        let config = load(&config_path).expect("load");
        assert_eq!(
            config.project.crate_root,
            Some(PathBuf::from("src/main.rs"))
        );
        assert_eq!(config.project.base_ref, "main");
        assert!(config.layers.annotation_ban);
        assert!(!config.layers.cross_reference);
        assert_eq!(config.filters.test_modules, vec!["tests"]);
    }

    #[test]
    fn load_with_all_defaults() {
        let dir = tempfile::tempdir().expect("tempdir");
        let config_path = dir.path().join("wire-check.toml");
        std::fs::write(&config_path, "").expect("write");

        let config = load(&config_path).expect("load");
        assert!(config.project.crate_root.is_none());
        assert_eq!(config.project.base_ref, "auto");
        assert!(config.layers.annotation_ban);
        assert!(config.layers.cross_reference);
        assert!(config.layers.dead_code_ratchet);
        assert!(config.layers.test_requirement);
    }

    #[test]
    fn defaults_returns_all_layers_enabled() {
        let config = defaults();
        assert!(config.layers.annotation_ban);
        assert!(config.layers.cross_reference);
        assert!(config.layers.dead_code_ratchet);
        assert!(config.layers.test_requirement);
    }

    #[test]
    fn load_nonexistent_file() {
        let result = load(Path::new("/tmp/does-not-exist-wire-check.toml"));
        assert!(matches!(result, Err(ConfigError::NotFound(_))));
    }

    #[test]
    fn load_invalid_toml() {
        let dir = tempfile::tempdir().expect("tempdir");
        let config_path = dir.path().join("wire-check.toml");
        std::fs::write(&config_path, "not valid { toml").expect("write");
        assert!(matches!(load(&config_path), Err(ConfigError::Parse(_))));
    }

    #[test]
    fn detect_crate_root_finds_main() {
        let dir = tempfile::tempdir().expect("tempdir");
        let src = dir.path().join("src");
        std::fs::create_dir(&src).expect("mkdir");
        std::fs::write(src.join("main.rs"), "fn main() {}").expect("write");

        let root = detect_crate_root(dir.path());
        assert!(root.is_some());
        assert!(root.unwrap().ends_with("src/main.rs"));
    }

    #[test]
    fn detect_crate_root_falls_back_to_lib() {
        let dir = tempfile::tempdir().expect("tempdir");
        let src = dir.path().join("src");
        std::fs::create_dir(&src).expect("mkdir");
        std::fs::write(src.join("lib.rs"), "").expect("write");

        let root = detect_crate_root(dir.path());
        assert!(root.is_some());
        assert!(root.unwrap().ends_with("src/lib.rs"));
    }

    #[test]
    fn detect_crate_root_returns_none_when_empty() {
        let dir = tempfile::tempdir().expect("tempdir");
        assert!(detect_crate_root(dir.path()).is_none());
    }
}
