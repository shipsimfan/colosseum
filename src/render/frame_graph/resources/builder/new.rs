use crate::render::frame_graph::{
    ArenaBuffer, FrameGraphExternalResource, FrameGraphResourceBuilder,
};
use alexandria::{gpu::VulkanImageView, math::Vector2u};

impl<'a> FrameGraphResourceBuilder<'a> {
    /// Create a new [`FrameGraphResourceBuilder`]
    pub fn new(
        external: &'a mut ArenaBuffer<FrameGraphExternalResource<'static>>,

        swapchain_image: &'a VulkanImageView,
        swapchain_image_size: Vector2u,
    ) -> FrameGraphResourceBuilder<'a> {
        let mut external = external.arena();
        external.push(FrameGraphExternalResource::new(
            swapchain_image,
            swapchain_image_size,
        ));

        FrameGraphResourceBuilder { external }
    }
}
