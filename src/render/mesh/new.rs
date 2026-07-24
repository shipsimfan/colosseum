use crate::{
    Result,
    render::{Mesh, MeshTransfer, RenderMesh, Vertex, transfer::GpuTransferQueue},
};
use alexandria::gpu::{VulkanBuffer, VulkanDeviceMemory};
use std::sync::Arc;

impl Mesh {
    /// Create a new [`Mesh`]
    pub(crate) fn new(
        vertices: Vec<Vertex>,
        indices: Vec<u32>,
        vertex_buffer: VulkanBuffer,
        index_buffer: VulkanBuffer,
        memory: Arc<VulkanDeviceMemory>,
        index_buffer_offset: u32,
        transfer_queue: &mut GpuTransferQueue,
    ) -> Result<(Arc<Mesh>, MeshTransfer)> {
        let mesh = Arc::new(Mesh { vertices, indices });
        let render_mesh = RenderMesh::new(vertex_buffer, index_buffer, memory, mesh.indices.len());

        let transfer = transfer_queue.transfer_mesh(&mesh, render_mesh, index_buffer_offset)?;

        Ok((mesh, transfer))
    }
}
