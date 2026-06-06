use crate::render::frame_graph::{FrameGraphStructure, structure::skybox::FrameGraphSkybox};

impl FrameGraphStructure {
    /// Get a reference to the skybox to be used in the frame graph structure
    pub fn skybox(&self) -> &FrameGraphSkybox {
        &self.skybox
    }
}
