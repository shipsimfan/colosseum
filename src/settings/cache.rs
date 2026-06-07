use crate::{Result, file_io::FileIo, logging::Logger, settings::DisplaySettings};
use std::path::Path;

/// A cache of settings groups
///
/// This trait should be implemented by adding the `#[colosseum::settings::settings_cache]`
/// attribute attached to a struct containing fields for each settings group, and implementing
/// `SettingsGroup` for each of those groups.
pub trait SettingsCache: Sized + Send {
    /// The settings cache that is modifiable
    type Modifiable: ModifiableSettingsCache;

    /// Loads the settings from `path`
    fn load(path: &Path, logger: Logger, file_io: &FileIo) -> Result<Self>;

    /// Create a modifiable version of this settings cache
    fn begin_modify(&self) -> Self::Modifiable;

    /// Save this settings cache to `path`
    fn save(&mut self, new_settings: &Self::Modifiable);

    /// Are the settings currently being saved?
    fn is_saving(&mut self) -> bool;

    /// Get the display settings
    fn display_settings(&self) -> &DisplaySettings;
}

/// A modifiable version of the settings cache
pub trait ModifiableSettingsCache {
    /// Get a mutable reference to the display settings
    fn display_settings_mut(&mut self) -> &mut DisplaySettings;
}
