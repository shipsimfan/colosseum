use crate::render::{RenderData, Skybox};

impl RenderData {
    /// Set the skybox to use for rendering
    pub fn set_skybox(&mut self, skybox: Skybox) {
        self.skybox = skybox;
    }
}
