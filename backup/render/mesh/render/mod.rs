use alexandria::gpu::{VulkanBuffer, VulkanDeviceMemory};
use std::sync::Arc;

mod bind;
mod get;
mod new;

/// A mesh on the GPU in the renderer
pub(crate) struct RenderMesh {
    /// The buffer for the vertices of the mesh
    vertex_buffer: VulkanBuffer,

    /// The buffer for the indices of the mesh
    index_buffer: VulkanBuffer,

    /// The device memory containing the index buffer
    #[allow(unused)]
    memory: Arc<VulkanDeviceMemory>,

    /// The number of indices in the mesh
    index_count: u32,
}
