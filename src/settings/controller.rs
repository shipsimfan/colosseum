use super::{LoadSettingsError, SaveSettingsError, Settings};
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

    /// Loads `T` from its configuration file
    pub fn load<T: Settings>(&mut self) -> Result<T, LoadSettingsError> {
        let path = self.directory.join(format!("{}.json", T::NAME));
        if !path
            .try_exists()
            .map_err(|error| LoadSettingsError::ReadFailed(error, path.clone()))?
        {
            let settings = T::default();
            self.save(&settings)?;
            return Ok(settings);
        }

        let contents = std::fs::read(&path)
            .map_err(|error| LoadSettingsError::ReadFailed(error, path.clone()))?;

        json::from_bytes(&contents)
            .map_err(|error| LoadSettingsError::DeserializeError(error.to_string(), path))
    }

    /// Saves `settings` to its configuration file
    pub fn save<T: Settings>(&mut self, settings: &T) -> Result<(), SaveSettingsError> {
        let path = self.directory.join(format!("{}.json", T::NAME));
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&path)
            .map_err(|error| SaveSettingsError::new(error.to_string(), path.clone()))?;
        json::to_write_pretty(settings, &mut file)
            .map_err(|error| SaveSettingsError::new(error.to_string(), path))
    }
}

impl !Send for SettingsController {}
