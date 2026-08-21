use crate::render::frame_graph::FrameGraphDynamicTransientResourceInfo;

impl FrameGraphDynamicTransientResourceInfo {
    /// Set this resource to be a color resource
    pub fn set_color(&mut self) {
        self.is_color = true;
    }

    /// Set this resource to be a depth resource
    pub fn set_depth(&mut self) {
        self.is_depth = true;
    }
}
