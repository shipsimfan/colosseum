use crate::render::{Mesh, transfer::SharedGpuTransferData};
use alexandria::gpu::VulkanBuffer;
use std::sync::Arc;

/// Transfer something to the GPU
pub(in crate::render::transfer) enum GpuTransferCommand {
    /// Transfer a mesh to the GPU
    Mesh {
        /// The mesh to be transferred
        mesh: Arc<Mesh>,

        /// The shared state for the transfer
        shared_state: Arc<SharedGpuTransferData>,

        /// The buffer to use for the vertex data
        vertex_buffer: VulkanBuffer,

        /// The buffer to use for the index data
        index_buffer: VulkanBuffer,
    },
}
