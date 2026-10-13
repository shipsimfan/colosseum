use crate::render::RenderMesh;
use alexandria::gpu::VulkanBuffer;

impl RenderMesh {
    /// Get the vertex buffer of the render mesh
    pub(in crate::render) fn vertex_buffer(&self) -> &VulkanBuffer {
        &self.vertex_buffer
    }

    /// Get the index buffer of the render mesh
    pub(in crate::render) fn index_buffer(&self) -> &VulkanBuffer {
        &self.index_buffer
    }

    /// Get the number of indices in the render mesh
    pub(in crate::render) fn index_count(&self) -> u32 {
        self.index_count
    }
}
