use crate::render::{RenderData, Skybox};

impl RenderData {
    /// Reset the render data to its default state for a new frame
    pub fn scene_reset(&mut self) {
        self.skybox = Skybox::default();
    }
}
