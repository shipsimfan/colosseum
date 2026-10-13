use crate::update::render_objects::allocator::GpuMemoryType;
use alexandria::gpu::VulkanDevice;

impl GpuMemoryType {
    /// Create a new [`GpuMemoryType`]
    pub fn new(
        memory_type_index: u32,
        chunk_size: u32,
        min_block_size: u32,
        max_block_size: u32,
        index: u8,
        device: VulkanDevice,
    ) -> GpuMemoryType {
        GpuMemoryType {
            chunks: Vec::new(),
            memory_type_index,
            chunk_size,
            min_block_size,
            max_block_size,
            index,
            device,
        }
    }
}
