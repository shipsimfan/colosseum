use crate::settings::DisplaySettings;
use alexandria::math::Vector2u;

impl DisplaySettings {
    /// Get the resolution to use for the display
    pub fn resolution(&self) -> Option<Vector2u> {
        self.resolution
    }

    /// Get whether to use fullscreen mode
    pub fn fullscreen(&self) -> bool {
        self.fullscreen
    }

    /// Get the name or UUID of the adapter to use for rendering
    pub fn adapter(&self) -> Option<&str> {
        self.adapter.as_deref()
    }
}
