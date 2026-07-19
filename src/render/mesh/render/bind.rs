use crate::render::RenderMesh;
use alexandria::gpu::{VulkanCommandBuffer, VulkanIndexType};

impl RenderMesh {
    /// Bind this mesh to the render pass
    pub(in crate::render) fn bind(&self, cmd_buffer: &mut VulkanCommandBuffer) {
        cmd_buffer.cmd_bind_vertex_buffer(0, &self.vertex_buffer, 0);
        cmd_buffer.cmd_bind_index_buffer(&self.index_buffer, 0, VulkanIndexType::Uint32);
    }
}
