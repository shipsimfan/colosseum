use crate::Result;
use std::path::Path;

/// A cache of settings groups
pub trait SettingsCache: Sized {
    /// Loads the setttings from `path`
    fn load(path: &Path) -> Result<Self>;
}
