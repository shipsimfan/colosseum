use crate::render::RenderMesh;
use alexandria::gpu::{VulkanBuffer, VulkanDeviceMemory};
use std::sync::Arc;

impl RenderMesh {
    /// Create a new [`RenderMesh`]
    pub fn new(
        vertex_buffer: VulkanBuffer,
        index_buffer: VulkanBuffer,
        memory: Arc<VulkanDeviceMemory>,
        index_count: usize,
    ) -> RenderMesh {
        RenderMesh {
            vertex_buffer,
            index_buffer,
            memory,
            index_count: index_count as u32,
        }
    }
}
