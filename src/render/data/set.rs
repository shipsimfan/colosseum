use crate::render::{AntiAliasingMode, RenderData, Skybox};

impl RenderData {
    /// Set the render scale to use for rendering
    pub fn set_render_scale(&mut self, render_scale: f32) {
        self.render_scale = render_scale;
    }

    /// Set the gamma to use for rendering
    pub fn set_gamma(&mut self, gamma: f32) {
        self.gamma = gamma;
    }

    /// Set the exposure to use for rendering
    pub fn set_exposure(&mut self, exposure: f32) {
        self.exposure = exposure;
    }

    /// Set the contrast to use for rendering
    pub fn set_contrast(&mut self, contrast: f32) {
        self.contrast = contrast;
    }

    /// Set the saturation to use for rendering
    pub fn set_saturation(&mut self, saturation: f32) {
        self.saturation = saturation;
    }

    /// Set the anti-aliasing mode to use for rendering
    pub fn set_anti_aliasing(&mut self, anti_aliasing: AntiAliasingMode) {
        self.anti_aliasing = anti_aliasing;
    }

    /// Set the skybox to use for rendering
    pub fn set_skybox(&mut self, skybox: Skybox) {
        self.skybox = skybox;
    }
}
