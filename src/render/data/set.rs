use crate::render::{RenderData, Skybox};

impl RenderData {
    /// Set the render scale to use for rendering
    pub fn set_render_scale(&mut self, render_scale: f32) {
        self.render_scale = render_scale;
    }

    /// Set the skybox to use for rendering
    pub fn set_skybox(&mut self, skybox: Skybox) {
        self.skybox = skybox;
    }
}
