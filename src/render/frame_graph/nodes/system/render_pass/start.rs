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

        let (color_attachment, mut size_and_layer_count) = match self.color_attachment {
            Some((color_attachment, load_op)) => {
                let color_attachment = resources.get(color_attachment, &frame_index);

                (
                    Some([VulkanRenderingAttachmentInfo::new(
                        color_attachment.image_view(),
                        VulkanImageLayout::ColorAttachmentOptimal,
                        VulkanResolveModeFlag::None,
                        None,
                        VulkanImageLayout::Undefined,
                        load_op,
                        VulkanAttachmentStoreOp::Store,
                        Color4f::<Linear>::new(0.0, 0.0, 0.0, 1.0),
                    )]),
                    Some((color_attachment.size(), color_attachment.layer_count())),
                )
            }
            None => (None, None),
        };

        let depth_attachment = match self.depth_attachment {
            Some((depth_attachment, load_op)) => {
                let depth_attachment = resources.get(depth_attachment, &frame_index);

                if size_and_layer_count.is_none() {
                    size_and_layer_count =
                        Some((depth_attachment.size(), depth_attachment.layer_count()));
                }

                Some(VulkanRenderingAttachmentInfo::new(
                    depth_attachment.image_view(),
                    VulkanImageLayout::DepthAttachmentOptimal,
                    VulkanResolveModeFlag::None,
                    None,
                    VulkanImageLayout::Undefined,
                    load_op,
                    VulkanAttachmentStoreOp::Store,
                    Color4f::<Linear>::new(0.0, 0.0, 0.0, 0.0),
                ))
            }
            None => None,
        };

        let (size, layer_count) = size_and_layer_count.unwrap();

        // Begin the render pass
        cmd_buffer.cmd_begin_rendering(
            0,
            Vector2i::ZERO,
            size,
            layer_count,
            0,
            match color_attachment.as_ref() {
                Some(color_attachment) => color_attachment,
                None => &[],
            },
            depth_attachment.as_ref(),
            None,
        );

        // Bind the viewport and scissor for the render pass
        let viewport = VulkanViewport::new(0.0, 0.0, size.x as _, size.y as _, 0.0, 1.0);
        let scissor = Recti::new(Vector2::ZERO, size);
        cmd_buffer.cmd_set_viewport(0, &[viewport]);
        cmd_buffer.cmd_set_scissor(0, &[scissor]);
    }
}
