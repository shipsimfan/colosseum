use crate::{
    Error, Result,
    file_io::{FileIo, WriteFullFile},
    info,
    logging::Logger,
};
use data_format::{Deserialize, Serialize};
use std::path::Path;

/// A named group of settings
pub trait SettingsGroup: for<'de> Deserialize<'de> + Serialize + Send + Clone + Default {
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
    unsafe fn load(path: &Path, logger: &Logger, file_io: &FileIo) -> Result<Self> {
        let path = path.join(format!("{}.json", Self::FILE_NAME));
        if !path.exists() {
            info!(
                logger,
                "No \"{}\" settings found at \"{}\", using defaults",
                Self::PRETTY_NAME,
                path.display(),
            );
            return Ok(Self::default());
        }

        let file = file_io.read_full_file_blocking(path.clone())?;

        let settings_group = json::from_bytes(&file).map_err(|error| {
            Error::new_with(format!("unable to load \"{}\"", path.display()), error)
        })?;

        info!(
            logger,
            "Loaded \"{}\" settings from \"{}\"",
            Self::PRETTY_NAME,
            path.display()
        );

        Ok(settings_group)
    }

    /// Save this settings group to a file under `path`
    ///
    /// Games should not directly call this function. Use the settings cache to save settings
    /// correctly.
    unsafe fn save(&self, path: &Path, logger: &Logger, file_io: &FileIo) -> WriteFullFile {
        let path = path.join(format!("{}.json", Self::FILE_NAME));

        let data = json::to_bytes_pretty(self).unwrap();

        info!(
            logger,
            "Saving \"{}\" settings to \"{}\"",
            Self::PRETTY_NAME,
            path.display()
        );
        file_io.write_full_file(path, data)
    }
}
