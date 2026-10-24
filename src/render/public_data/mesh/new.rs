use crate::{
    Result,
    render::{GpuAllocatedMemory, LoadingMesh, Mesh, Vertex, transfer::GpuTransferQueue},
};
use alexandria::gpu::VulkanBuffer;

impl Mesh {
    /// Create a new [`Mesh`]
    pub(in crate::render::public_data) fn new(
        vertices: Vec<Vertex>,
        indices: Vec<u32>,
        vertex_buffer: VulkanBuffer,
        index_buffer: VulkanBuffer,
        memory: GpuAllocatedMemory,
        transfer_queue: &GpuTransferQueue,
    ) -> Result<LoadingMesh> {
        let mesh = Mesh {
            vertices,
            indices,
            vertex_buffer,
            index_buffer,
            memory,
        };

        let loading_mesh = transfer_queue.transfer_mesh(mesh)?;

        Ok(loading_mesh)
    }
}
