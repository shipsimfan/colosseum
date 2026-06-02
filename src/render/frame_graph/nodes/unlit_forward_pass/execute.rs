use crate::render::{
    RenderData,
    frame_graph::{FrameGraphResources, UnlitForwardPassNode},
};
use alexandria::gpu::VulkanCommandBuffer;

impl UnlitForwardPassNode {
    /// Execute the unlit forward pass, rendering all unlit objects in the scene
    pub(in crate::render::frame_graph) fn execute(
        &self,
        render_data: &RenderData,
        resources: &FrameGraphResources,
        cmd_buffer: &mut VulkanCommandBuffer,
    ) {
        let clear_color = render_data.clear_color().with_alpha(1.0);
        let resource = &resources[self.output];

        cmd_buffer.cmd_begin_rendering(resource.image_view(), resource.size(), clear_color);

        cmd_buffer.cmd_end_rendering();
    }
}
