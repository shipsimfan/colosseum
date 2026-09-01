use crate::render::frame_graph::{FrameGraphResourceId, FrameGraphResourceUsage, ToneMapNode};

impl ToneMapNode {
    /// Get the usage types for the resources that this node uses
    pub(in crate::render::frame_graph::nodes) fn usages<
        T,
        F: FnOnce(&[(FrameGraphResourceId, FrameGraphResourceUsage)]) -> T,
    >(
        &self,
        f: F,
    ) -> T {
        f(&[
            (self.input, FrameGraphResourceUsage::SampledImage),
            (self.output, FrameGraphResourceUsage::ColorAttachment),
        ])
    }
}
