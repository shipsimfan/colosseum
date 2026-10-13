use crate::render::{
    AntiAliasingMode,
    frame_graph::{FrameGraphStructure, structure::skybox::FrameGraphSkybox},
};

impl FrameGraphStructure {
    /// Does the frame graph structure have a render scale?
    pub fn has_render_scale(&self) -> bool {
        self.has_render_scale
    }

    /// Get a reference to the skybox to be used in the frame graph structure
    pub fn skybox(&self) -> &FrameGraphSkybox {
        &self.skybox
    }

    /// Get the anti-aliasing mode to be used in the frame graph structure
    pub fn anti_aliasing(&self) -> AntiAliasingMode {
        self.anti_aliasing
    }
}
