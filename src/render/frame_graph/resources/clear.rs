use crate::render::frame_graph::FrameGraphResources;

impl FrameGraphResources {
    /// Clear all external resources from the frame graph
    pub fn clear(&mut self) {
        self.external_resources = Vec::new();
    }
}
