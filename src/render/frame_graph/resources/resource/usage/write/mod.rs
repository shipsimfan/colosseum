use alexandria::gpu::VulkanAttachmentLoadOp;

/// How a resource is written to by a node
pub(in crate::render::frame_graph) enum FrameGraphResourceWriteUsage {
    /// The resource is written to as a color attachment
    ColorAttachment { load_op: VulkanAttachmentLoadOp },
}
