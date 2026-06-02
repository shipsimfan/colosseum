use crate::render::frame_graph::{FrameGraphResource, FrameGraphResourceId, FrameGraphResources};
use alexandria::{
    gpu::{VulkanFormat, VulkanImageView},
    math::Vector2u,
};

impl<'a> FrameGraphResources<'a> {
    /// Register a new external resource in the frame graph, which can be accessed by nodes during execution
    pub fn register(
        &mut self,
        image: &'a VulkanImageView,
        size: Vector2u,
        format: VulkanFormat,
    ) -> FrameGraphResourceId {
        let resource = FrameGraphResource::new_external(image, size, format);
        self.external.push(resource);
        FrameGraphResourceId::new_external(self.external.len() - 1)
    }
}
