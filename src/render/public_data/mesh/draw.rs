use crate::render::Mesh;
use alexandria::gpu::{VulkanCommandBuffer, VulkanIndexType};

impl Mesh {
    /// Draw this mesh using the provided command buffer
    pub(in crate::render) fn draw(&self, cmd_buffer: &mut VulkanCommandBuffer) {
        cmd_buffer.cmd_bind_vertex_buffer(0, &self.vertex_buffer, 0);
        cmd_buffer.cmd_bind_index_buffer(&self.index_buffer, 0, VulkanIndexType::Uint32);
        cmd_buffer.cmd_draw_indexed(self.indices.len() as _, 1, 0, 0, 0);
    }
}
