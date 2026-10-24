use crate::render::{GpuAllocatedMemory, Mesh, Vertex};
use alexandria::gpu::VulkanBuffer;

impl Mesh {
    /// Get the vertices of the mesh
    pub fn vertices(&self) -> &[Vertex] {
        &self.vertices
    }

    /// Get the indices of the mesh
    pub fn indices(&self) -> &[u32] {
        &self.indices
    }

    /// Get a reference to the vertex buffer of the mesh
    pub(in crate::render) fn vertex_buffer(&self) -> &VulkanBuffer {
        &self.vertex_buffer
    }

    /// Get a reference to the index buffer of the mesh
    pub(in crate::render) fn index_buffer(&self) -> &VulkanBuffer {
        &self.index_buffer
    }

    /// Get a reference to the device memory containing the buffers
    pub(in crate::render) fn memory(self) -> GpuAllocatedMemory {
        self.memory
    }
}
