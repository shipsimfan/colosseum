use crate::{Result, logging::Logger};
use std::path::Path;

/// A cache of settings groups
///
/// This trait should be implemented by adding the `#[colosseum::settings::settings_cache]`
/// attribute attached to a struct containing fields for each settings group, and implementing
/// `SettingsGroup` for each of those groups.
pub trait SettingsCache: Sized {
    /// Loads the setttings from `path`
    fn load(path: &Path, logger: Logger) -> Result<Self>;

    /// Saves all the settings to `path`
    fn save(&self, path: &Path) -> Result<()>;
}
