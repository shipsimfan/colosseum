use crate::render::frame_graph::{
    ArenaBuffer, FrameGraphDynamicTransientResourceInfo, FrameGraphExternalResource,
    FrameGraphResourceBuilder,
};
use alexandria::{
    gpu::{VulkanImage, VulkanImageAspectFlag, VulkanImageView},
    math::Vector2u,
};

impl<'a> FrameGraphResourceBuilder<'a> {
    /// Create a new [`FrameGraphResourceBuilder`]
    pub fn new(
        external: &'a mut ArenaBuffer<FrameGraphExternalResource<'static>>,
        transient_render_scale: &'a mut Vec<FrameGraphDynamicTransientResourceInfo>,
        transient_native_scale: &'a mut Vec<FrameGraphDynamicTransientResourceInfo>,

        swapchain_size: Vector2u,
        swapchain_image: &'a VulkanImage,
        swapchain_image_view: &'a VulkanImageView,
    ) -> FrameGraphResourceBuilder<'a> {
        let mut external = external.arena();
        external.push(FrameGraphExternalResource::new(
            swapchain_size,
            swapchain_image,
            swapchain_image_view,
            VulkanImageAspectFlag::Color,
        ));

        FrameGraphResourceBuilder {
            external,
            transient_render_scale,
            transient_native_scale,
        }
    }
}
