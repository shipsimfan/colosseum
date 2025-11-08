use crate::{Error, Result};
use data_format::{Deserialize, Serialize};
use std::path::Path;

/// A named group of settings
pub trait SettingsGroup: for<'de> Deserialize<'de> + Serialize + Clone + Send {
    /// The name of the file to load and save these settings to
    const FILE_NAME: &str;

    /// Load this settings group from a file under `path`
    ///
    /// Games should not directly call this function. Use the settings cache to read settings
    /// correctly.
    unsafe fn load(path: &Path) -> Result<Self> {
        std::fs::create_dir_all(path).map_err(|error| {
            Error::new_inner(format!("unable to create \"{}\"", path.display()), error)
        })?;

        let path = path.join(format!("{}.json", Self::FILE_NAME));
        if !path.exists() {
            return json::from_str("{}").map_err(|error| {
                Error::new_inner(format!("unable to load \"{}\"", path.display()), error)
            });
        }

        let file = std::fs::read(&path).map_err(|error| {
            Error::new_inner(format!("unable to load \"{}\"", path.display()), error)
        })?;

        json::from_bytes(&file).map_err(|error| {
            Error::new_inner(format!("unable to load \"{}\"", path.display()), error)
        })
    }

    /// Save this settings group to a file under `path`
    ///
    /// Games should not directly call this function. Use the settings cache to save settings
    /// correctly.
    unsafe fn save(&self, path: &Path) -> Result<()> {
        let path = path.join(format!("{}.json", Self::FILE_NAME));
        let file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&path)
            .map_err(|error| {
                Error::new_inner(format!("unable to save \"{}\"", path.display()), error)
            })?;

        match json::to_write_pretty(self, file) {
            Ok(()) => Ok(()),
            Err(json::SerializeError::IO(error)) => Err(Error::new_inner(
                format!("unable to save \"{}\"", path.display()),
                error,
            )),
            Err(json::SerializeError::Custom(_)) => {
                panic!("unable to save \"{}\"", path.display())
            }
        }
    }
}
