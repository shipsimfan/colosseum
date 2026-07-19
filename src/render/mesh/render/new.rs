use crate::render::RenderMesh;
use alexandria::gpu::{VulkanBuffer, VulkanDeviceMemory};
use std::sync::Arc;

impl RenderMesh {
    /// Create a new [`RenderMesh`]
    pub fn new(
        vertex_buffer: VulkanBuffer,
        vertex_buffer_memory: Arc<VulkanDeviceMemory>,
        index_buffer: VulkanBuffer,
        index_buffer_memory: Arc<VulkanDeviceMemory>,
        index_count: usize,
    ) -> RenderMesh {
        RenderMesh {
            vertex_buffer,
            vertex_buffer_memory,
            index_buffer,
            index_buffer_memory,
            index_count: index_count as u32,
        }
    }
}
