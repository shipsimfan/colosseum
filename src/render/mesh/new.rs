use crate::{
    Result,
    render::{Mesh, MeshTransfer, Vertex, transfer::GpuTransferQueue},
};
use alexandria::gpu::VulkanBuffer;
use std::sync::Arc;

impl Mesh {
    /// Create a new [`Mesh`]
    pub(crate) fn new(
        vertices: Vec<Vertex>,
        indices: Vec<u32>,
        vertex_buffer: VulkanBuffer,
        index_buffer: VulkanBuffer,
        transfer_queue: &mut GpuTransferQueue,
    ) -> Result<(Arc<Mesh>, MeshTransfer)> {
        let mesh = Arc::new(Mesh { vertices, indices });

        let transfer = transfer_queue.transfer_mesh(&mesh, vertex_buffer, index_buffer)?;

        Ok((mesh, transfer))
    }
}
