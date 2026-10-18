use crate::render::frame_graph::{
    FrameGraphExternalResource, FrameGraphResourceId, FrameGraphResources,
};
use alexandria::{
    gpu::{VulkanImageAspectFlags, VulkanImageView},
    math::Vector2u,
};
use std::sync::Arc;

impl FrameGraphResources {
    /// Insert a new external resource into the frame graph
    pub fn insert_external<A: Into<VulkanImageAspectFlags>>(
        &mut self,
        id: FrameGraphResourceId,
        image_view: &Arc<VulkanImageView>,
        size: Vector2u,
        layer_count: u32,
        aspect_mask: A,
    ) {
        debug_assert!(id.is_external());
        debug_assert!(self.external_resources.len() <= id.index());

        let resource = FrameGraphExternalResource::new(
            image_view.clone(),
            size,
            layer_count,
            aspect_mask.into(),
        );

        if self.external_resources.len() < id.index() {
            self.external_resources[id.index()] = resource;
        } else {
            self.external_resources.push(resource);
        }
    }
}
