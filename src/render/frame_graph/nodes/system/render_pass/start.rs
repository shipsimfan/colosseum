use crate::render::{
    FrameIndex,
    frame_graph::{FrameGraphResources, nodes::system::FrameGraphRenderPass},
};
use alexandria::{
    gpu::{
        VulkanAttachmentStoreOp, VulkanCommandBuffer, VulkanImageLayout,
        VulkanRenderingAttachmentInfo, VulkanResolveModeFlag, VulkanViewport,
    },
    math::{Color4f, Linear, Recti, Vector2, Vector2i},
};
use colosseum_ecs::UnsafeWorldCell;

impl FrameGraphRenderPass {
    /// Start the render pass
    pub fn start(&self, cmd_buffer: &mut VulkanCommandBuffer, world: &UnsafeWorldCell) {
        let resources = unsafe {
            world
                .get()
                .get_resource_at_index::<FrameGraphResources>(self.resources)
        };
        let frame_index = unsafe {
            world
                .get()
                .get_resource_at_index::<FrameIndex>(self.frame_index)
        };

        let color_attachment = resources.get(self.color_attachment, &frame_index);
        let size = color_attachment.size();

        // Begin the render pass
        cmd_buffer.cmd_begin_rendering(
            0,
            Vector2i::ZERO,
            size,
            1,
            0,
            &[VulkanRenderingAttachmentInfo::new(
                color_attachment.image_view(),
                VulkanImageLayout::ColorAttachmentOptimal,
                VulkanResolveModeFlag::None,
                None,
                VulkanImageLayout::Undefined,
                self.color_attachment_load_op,
                VulkanAttachmentStoreOp::Store,
                Color4f::<Linear>::new(0.0, 0.0, 0.0, 1.0),
            )],
            None,
            None,
        );

        // Bind the viewport and scissor for the render pass
        let viewport = VulkanViewport::new(0.0, 0.0, size.x as _, size.y as _, 0.0, 1.0);
        let scissor = Recti::new(Vector2::ZERO, size);
        cmd_buffer.cmd_set_viewport(0, &[viewport]);
        cmd_buffer.cmd_set_scissor(0, &[scissor]);
    }
}
