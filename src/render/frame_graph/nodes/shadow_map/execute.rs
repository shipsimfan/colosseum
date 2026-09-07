use crate::render::{
    RenderData, RenderObjects,
    frame_graph::{FrameGraphResources, ShadowMapNode},
};
use alexandria::gpu::VulkanCommandBuffer;

impl ShadowMapNode {
    /// Execute the lit forward pass, rendering all lit objects in the scene
    pub(in crate::render::frame_graph::nodes) fn execute(
        &self,
        render_data: &RenderData,
        render_objects: &RenderObjects,
        resources: &FrameGraphResources,
        cmd_buffer: &mut VulkanCommandBuffer,
    ) {
        todo!("Execute shadow map passes for each light")
    }
}
