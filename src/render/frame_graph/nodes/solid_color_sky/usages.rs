use crate::render::frame_graph::{
    FrameGraphResourceId, FrameGraphResourceUsage, SolidColorSkyNode,
};

impl SolidColorSkyNode {
    /// Get the usage types for the resources that this node uses
    pub(in crate::render::frame_graph::nodes) fn usages<
        T,
        F: FnOnce(&[(FrameGraphResourceId, FrameGraphResourceUsage)]) -> T,
    >(
        &self,
        f: F,
    ) -> T {
        f(&[
            (self.output, FrameGraphResourceUsage::ColorAttachment),
            (self.depth_buffer, FrameGraphResourceUsage::DepthAttachment),
        ])
    }
}
