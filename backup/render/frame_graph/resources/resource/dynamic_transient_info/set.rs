use crate::render::frame_graph::FrameGraphDynamicTransientResourceInfo;

impl FrameGraphDynamicTransientResourceInfo {
    /// Set this resource to be a color resource
    pub fn set_color(&mut self) {
        assert!(!self.is_depth);
        self.is_color = true;
    }

    /// Set this resource to be a depth resource
    pub fn set_depth(&mut self) {
        assert!(!self.is_color);
        self.is_depth = true;
    }

    /// Set this resource to be a transfer destination resource
    pub fn set_transfer_dst(&mut self) {
        self.is_transfer_dst = true;
    }

    /// Set this resource to be a transfer source resource
    pub fn set_transfer_src(&mut self) {
        self.is_transfer_src = true;
    }

    /// Set this resource to be a sampled image resource
    pub fn set_sampled_image(&mut self) {
        self.is_sampled_image = true;
    }
}
