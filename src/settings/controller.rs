use std::{
    convert::Infallible,
    path::{Path, PathBuf},
};

use super::Settings;

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

    /// Loads `T` from its configuration file
    pub fn load<T: Settings>(&mut self) -> Result<T, Infallible> {
        todo!()
    }

    /// Saves `settings` to its configuration file
    pub fn save<T: Settings>(&mut self, settings: T) -> Result<(), Infallible> {
        todo!()
    }
}

impl !Send for SettingsController {}
