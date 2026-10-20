use crate::render::frame_graph::{FrameGraphResourceId, nodes::system::FrameGraphRenderPass};

impl FrameGraphRenderPass {
    /// Create a new [`FrameGraphRenderPass`]
    pub fn new(color_attachment: Option<FrameGraphResourceId>) -> Option<FrameGraphRenderPass> {
        Some(FrameGraphRenderPass {
            resources: 0,
            color_attachment: color_attachment?,
        })
    }
}
