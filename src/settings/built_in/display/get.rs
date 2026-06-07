use crate::settings::DisplaySettings;
use alexandria::math::{Vector2i, Vector2u};

impl DisplaySettings {
    /// Get the position to display the window at
    pub fn position(&self) -> Option<Vector2i> {
        self.position
    }

    /// Get the resolution to use for the display
    pub fn resolution(&self) -> Option<Vector2u> {
        self.resolution
    }

    /// Get whether to use fullscreen mode
    pub fn fullscreen(&self) -> bool {
        self.fullscreen
    }

    /// Get whether the window is maximized
    pub fn maximized(&self) -> bool {
        self.maximized
    }

    /// Get the name or UUID of the adapter to use for rendering
    pub fn adapter(&self) -> Option<&str> {
        self.adapter.as_deref()
    }
}
