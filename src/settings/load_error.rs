use super::SaveSettingsError;
use std::{
    fmt::{Display, Formatter},
    path::PathBuf,
};

/// An error that occurred while loading a settings file
#[derive(Debug)]
pub enum LoadSettingsError {
    /// Reading the file failed
    ReadFailed(std::io::Error, PathBuf),

    /// Deserializing the file failed
    DeserializeError(String, PathBuf),

    /// Saving a default file back failed
    SaveFailed(SaveSettingsError),
}

impl alexandria::Error for LoadSettingsError {
    fn title(&self) -> &'static str {
        "Load Settings Error"
    }
}

impl std::error::Error for LoadSettingsError {}

impl Display for LoadSettingsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LoadSettingsError::ReadFailed(error, path) => {
                write!(f, "Failed to read \"{}\" - {}", path.display(), error)
            }
            LoadSettingsError::DeserializeError(error, path) => {
                write!(f, "Failed to parse \"{}\" - {}", path.display(), error)
            }
            LoadSettingsError::SaveFailed(error) => error.fmt(f),
        }
    }
}

impl From<SaveSettingsError> for LoadSettingsError {
    fn from(error: SaveSettingsError) -> Self {
        LoadSettingsError::SaveFailed(error)
    }
}
