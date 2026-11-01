use crate::render::{
    AntiAliasing,
    frame_graph::{FrameGraphStructure, SkyboxType},
};

impl FrameGraphStructure {
    /// Is render scaling required?
    pub fn render_scale(&self) -> bool {
        self.render_scale
    }

    /// Get the anti-aliasing mode of the frame graph structure
    pub fn anti_aliasing(&self) -> AntiAliasing {
        self.anti_aliasing
    }

    /// Get the skybox type of the frame graph structure
    pub fn skybox(&self) -> SkyboxType {
        self.skybox
    }
}
