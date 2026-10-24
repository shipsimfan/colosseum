use crate::{DisplaySettings, render::AntiAliasing};
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
    pub fn set_render_scale(&mut self, render_scale: f32) {
        self.render_scale = render_scale;
    }

    /// Set the gamma to use for rendering
    pub fn set_gamma(&mut self, gamma: f32) {
        self.gamma = gamma;
    }

    /// Set the anti-aliasing mode to use for rendering
    pub fn set_anti_aliasing(&mut self, anti_aliasing: AntiAliasing) {
        self.anti_aliasing = anti_aliasing;
    }

    /*
    /// Set the shadow quality to use for rendering
    pub fn set_shadow_quality(&mut self, shadow_quality: ShadowQuality) {
        self.shadow_quality = shadow_quality;
    }
    */

    /// Set the frame rate to run the game at when minimized
    pub fn set_minimized_frame_rate(&mut self, minimized_frame_rate: f32) {
        self.minimized_frame_rate = minimized_frame_rate;
    }

    /// Set the maximum frame rate to run the game at
    pub fn set_maximum_frame_rate(&mut self, maximum_frame_rate: Option<f32>) {
        self.maximum_frame_rate = maximum_frame_rate;
    }
}
