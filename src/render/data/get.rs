use crate::render::{RenderData, Skybox};

impl RenderData {
    /// Get the skybox to render
    pub fn skybox(&self) -> &Skybox {
        &self.skybox
    }
}
