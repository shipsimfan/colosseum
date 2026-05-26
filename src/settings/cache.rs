use crate::{Result, logging::Logger};
use std::path::Path;

/// A cache of settings groups
///
/// This trait should be implemented by adding the `#[colosseum::settings::settings_cache]`
/// attribute attached to a struct containing fields for each settings group, and implementing
/// `SettingsGroup` for each of those groups.
pub trait SettingsCache: Sized {
    /// The settings cache that is modifiable
    type Modifiable;

    /// Loads the settings from `path`
    fn load(path: &Path, logger: Logger) -> Result<Self>;

    /// Create a modifiable version of this settings cache
    fn begin_modify(&self) -> Self::Modifiable;

    /// Save this settings cache to `path`
    fn save(&mut self, new_settings: &Self::Modifiable) -> Result<()>;
}
