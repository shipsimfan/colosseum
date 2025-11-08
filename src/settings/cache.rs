use crate::{Result, graphics::GraphicsSettings};
use std::path::Path;

/// A cache of settings groups
pub trait SettingsCache: Sized {
    /// Loads the setttings from `path`
    fn load(path: &Path) -> Result<Self>;

    /// Gets the settings controlling the graphics subsystem
    fn graphics_settings(&self) -> &GraphicsSettings;
}
