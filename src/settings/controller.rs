use super::{LoadSettingsError, SaveSettingsError, Settings};
use crate::{
    error, info,
    logging::{LogController, Logger},
};
use std::path::{Path, PathBuf};

/// Loads and saves settings
pub struct SettingsController {
    /// The configuration directory
    directory: PathBuf,

    /// The logger for changes to settings
    logger: Logger,
}

const DEFAULT_CONFIG_DIR: &str = "config/";

/// Creates the directory for settings if it does not already exist
fn create_directory(directory: &Path, logger: &Logger) {
    if Path::new(directory).try_exists().unwrap_or(false) {
        return;
    }

    if let Err(error) = std::fs::create_dir(directory) {
        error!(
            logger,
            "Failed to create \"{}\" - {}",
            directory.display(),
            error
        )
    }
}

impl SettingsController {
    /// Creates a new [`SettingsController`]
    pub(crate) fn new(directory: Option<&Path>, log_controller: &LogController) -> Self {
        let logger = log_controller.logger("settings");

        let directory = directory
            .unwrap_or(Path::new(DEFAULT_CONFIG_DIR))
            .to_path_buf();
        create_directory(&directory, &logger);

        SettingsController { directory, logger }
    }

    /// Loads `T` from its configuration file
    pub fn load<T: Settings>(&mut self) -> Result<T, LoadSettingsError> {
        info!(self.logger, "Loading {}", T::NAME);

        let path = self.directory.join(format!("{}.json", T::NAME));
        if !path.try_exists().unwrap_or(false) {
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
        info!(self.logger, "Saving {}", T::NAME);

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
