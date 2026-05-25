use crate::{Error, Result, info, logging::Logger};
use data_format::{Deserialize, Serialize};
use std::path::Path;

/// A named group of settings
pub trait SettingsGroup: for<'de> Deserialize<'de> + Serialize + Send + Default {
    /// The name of the file to load and save these settings to
    const FILE_NAME: &str;

    /// A human-readable name for these settings, used in the UI and logs
    const PRETTY_NAME: &str;

    /// Gets the file name for these settings
    fn file_name(&self) -> &'static str {
        Self::FILE_NAME
    }

    /// Gets the pretty name for these settings
    fn pretty_name(&self) -> &'static str {
        Self::PRETTY_NAME
    }

    /// Load this settings group from a file under `path`
    ///
    /// Games should not directly call this function. Use the settings cache to read settings
    /// correctly.
    unsafe fn load(path: &Path, logger: &Logger) -> Result<Self> {
        let path = path.join(format!("{}.json", Self::FILE_NAME));
        if !path.exists() {
            info!(
                logger,
                "no \"{}\" settings found at \"{}\", using defaults",
                Self::PRETTY_NAME,
                path.display(),
            );
            return Ok(Self::default());
        }

        let file = std::fs::read(&path).map_err(|error| {
            Error::new_with(format!("unable to load \"{}\"", path.display()), error)
        })?;

        let settings_group = json::from_bytes(&file).map_err(|error| {
            Error::new_with(format!("unable to load \"{}\"", path.display()), error)
        })?;

        info!(
            logger,
            "loaded \"{}\" settings from \"{}\"",
            Self::PRETTY_NAME,
            path.display()
        );

        Ok(settings_group)
    }

    /// Save this settings group to a file under `path`
    ///
    /// Games should not directly call this function. Use the settings cache to save settings
    /// correctly.
    unsafe fn save(&self, path: &Path, logger: &Logger) -> Result<()> {
        let path = path.join(format!("{}.json", Self::FILE_NAME));
        let file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&path)
            .map_err(|error| {
                Error::new_with(format!("unable to save \"{}\"", path.display()), error)
            })?;

        match json::to_write_pretty(self, file) {
            Ok(()) => Ok(()),
            Err(json::SerializeError::IO(error)) => Err(Error::new_with(
                format!("unable to save \"{}\"", path.display()),
                error,
            )),
            Err(json::SerializeError::Custom(error)) => Err(Error::new_with(
                format!("unable to save \"{}\"", path.display()),
                error,
            )),
        }?;

        info!(
            logger,
            "saved \"{}\" settings to \"{}\"",
            Self::PRETTY_NAME,
            path.display()
        );
        Ok(())
    }
}
