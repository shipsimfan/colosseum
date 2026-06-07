use crate::settings::DisplaySettings;
use alexandria::math::Vector2u;

impl DisplaySettings {
    /// Set the resolution to use for the display
    pub(crate) fn set_resolution(&mut self, resolution: Vector2u) {
        self.resolution = Some(resolution);
    }
}
