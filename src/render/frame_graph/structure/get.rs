use crate::render::{AntiAliasing, frame_graph::FrameGraphStructure};

impl FrameGraphStructure {
    /// Is render scaling required?
    pub fn render_scale(&self) -> bool {
        self.render_scale
    }

    /// Get the anti-aliasing mode of the frame graph structure
    pub fn anti_aliasing(&self) -> AntiAliasing {
        self.anti_aliasing
    }
}
