use crate::render::frame_graph::{FrameGraphResourceId, FrameGraphResourceUsage, RenderScaleNode};

impl RenderScaleNode {
    /// Get the usage types for the resources that this node uses
    pub(in crate::render::frame_graph) fn usages<
        T,
        F: FnOnce(&[(FrameGraphResourceId, FrameGraphResourceUsage)]) -> T,
    >(
        &self,
        f: F,
    ) -> T {
        f(&[
            (self.input, FrameGraphResourceUsage::TransferSrc),
            (self.output, FrameGraphResourceUsage::TransferDst),
        ])
    }
}
