use std::{fmt::Display, path::PathBuf};

/// An error that occurred while saving settings
#[derive(Debug)]
pub struct SaveSettingsError {
    /// The description of the error
    error: String,

    /// The file which the error happened with
    path: PathBuf,
}

impl SaveSettingsError {
    /// Creates a new [`SaveSettingsError`]
    pub(super) fn new(error: String, path: PathBuf) -> Self {
        SaveSettingsError { error, path }
    }
}

impl alexandria::Error for SaveSettingsError {
    fn title(&self) -> &'static str {
        "Save Settings Error"
    }
}

impl std::error::Error for SaveSettingsError {}

impl Display for SaveSettingsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Failed to save \"{}\" - {}",
            self.path.display(),
            self.error
        )
    }
}
