use crate::render::{RenderData, Skybox};

impl RenderData {
    /// Set the skybox to render
    pub fn set_skybox<S: Into<Skybox>>(&mut self, skybox: S) {
        self.skybox = skybox.into();
    }
}
