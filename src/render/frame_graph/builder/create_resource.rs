use crate::render::frame_graph::{FrameGraphBuilder, FrameGraphResourceId, FrameGraphResourceKind};
use alexandria::gpu::{VulkanAttachmentLoadOp, VulkanFormat};

impl<'a> FrameGraphBuilder<'a> {
    /// Create a new transient resource
    pub fn create_resource(
        &mut self,
        name: &'static str,
        format: VulkanFormat,
        kind: FrameGraphResourceKind,
        initial_load_op: VulkanAttachmentLoadOp,
    ) -> FrameGraphResourceId {
        self.resources
            .create_resource(name, format, kind, initial_load_op)
    }
}
