use crate::render::{
    RenderObjects,
    frame_graph::{FrameGraphResources, RenderScaleNode},
};
use alexandria::gpu::VulkanDevice;

impl RenderScaleNode {
    /// Update the descriptor sets for this node
    pub(in crate::render::frame_graph::nodes) fn update_descriptor_sets(
        &self,
        _: &RenderObjects,
        _: &FrameGraphResources,
        _: &VulkanDevice,
    ) {
    }
}
