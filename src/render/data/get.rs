use crate::render::{RenderData, Skybox};

impl RenderData {
    /// Get a reference to the skybox to render
    pub fn skybox(&self) -> &Skybox {
        &self.skybox
    }

    /// Get a mutable reference to the skybox to render
    pub fn skybox_mut(&mut self) -> &mut Skybox {
        &mut self.skybox
    }
}
