use crate::render::frame_graph::{
    FrameGraphResourceId, FrameGraphResourceLoadOp, FrameGraphResourceWriteUsage, SolidColorSkyNode,
};

impl SolidColorSkyNode {
    /// Get the resources that this node writes to
    pub(in crate::render::frame_graph) fn write_resources<
        T,
        F: FnOnce(&[(FrameGraphResourceId, FrameGraphResourceWriteUsage)]) -> T,
    >(
        &self,
        f: F,
    ) -> T {
        f(&[(
            self.output,
            FrameGraphResourceWriteUsage::ColorAttachment {
                load_op: FrameGraphResourceLoadOp::Clear {
                    color: self.color.with_alpha(1.0),
                },
            },
        )])
    }
}
