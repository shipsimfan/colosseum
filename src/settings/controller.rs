use std::path::{Path, PathBuf};

/// Loads and saves settings
pub struct SettingsController {
    /// The configuration directory
    directory: PathBuf,
}

const DEFAULT_CONFIG_DIR: &str = "config/";

impl SettingsController {
    /// Creates a new [`SettingsController`]
    pub(crate) fn new(directory: Option<&Path>) -> Self {
        let directory = directory
            .unwrap_or(Path::new(DEFAULT_CONFIG_DIR))
            .to_path_buf();

        SettingsController { directory }
    }
}

impl !Send for SettingsController {}
