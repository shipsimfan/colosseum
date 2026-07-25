use crate::{
    Result,
    render::{Mesh, MeshTransfer, RenderMesh, Vertex, transfer::GpuTransferQueue},
    update::GpuAllocatedMemory,
};
use alexandria::gpu::VulkanBuffer;

impl Mesh {
    /// Create a new [`Mesh`]
    pub(crate) fn new(
        vertices: Vec<Vertex>,
        indices: Vec<u32>,
        vertex_buffer: VulkanBuffer,
        index_buffer: VulkanBuffer,
        allocation: GpuAllocatedMemory,
        transfer_queue: &mut GpuTransferQueue,
    ) -> Result<MeshTransfer> {
        let mesh = Mesh { vertices, indices };
        let render_mesh = RenderMesh::new(
            vertex_buffer,
            index_buffer,
            allocation.device_memory().clone(),
            mesh.indices.len(),
        );

        let transfer = transfer_queue.transfer_mesh(mesh, render_mesh, allocation)?;

        Ok(transfer)
    }
}
