use crate::settings::DisplaySettings;
use alexandria::math::{Vector2i, Vector2u};

impl DisplaySettings {
    /// Set the position to display the window at
    pub(crate) fn set_position(&mut self, position: Vector2i) {
        self.position = Some(position);
    }

    /// Set the resolution to use for the display
    pub(crate) fn set_resolution(&mut self, resolution: Vector2u) {
        self.resolution = Some(resolution);
    }

    /// Set whether to use fullscreen mode
    pub(crate) fn set_fullscreen(&mut self, fullscreen: bool) {
        self.fullscreen = fullscreen;
    }

    /// Set whether the window is maximized
    pub(crate) fn set_maximized(&mut self, maximized: bool) {
        self.maximized = maximized;
    }

    /// Set the render scale to use for rendering
    pub(crate) fn set_render_scale(&mut self, render_scale: f32) {
        self.render_scale = render_scale;
    }
}
