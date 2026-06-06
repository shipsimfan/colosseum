use crate::render::{
    FrameGraph, FrameGraphResourceId, RenderData, frame_graph::FrameGraphResources,
};
use alexandria::{
    gpu::{VulkanFormat, VulkanImageView},
    math::Vector2u,
};

impl FrameGraph {
    /// Build the frame graph for a single frame
    pub(in crate::render::frame_graph) fn build<'a>(
        &mut self,
        data: &RenderData,
        swapchain_image: &'a VulkanImageView,
        swapchain_image_size: Vector2u,
        swapchain_image_format: VulkanFormat,
        resources: &mut FrameGraphResources<'a>,
    ) -> FrameGraphResourceId {
        // Register the swapchain image as a resource in the frame graph
        let swapchain_image = resources.register(
            swapchain_image,
            swapchain_image_size,
            swapchain_image_format,
        );

        // Add nodes to the frame graph
        self.add_node(data.skybox().create_node(swapchain_image));

        swapchain_image
    }
}
