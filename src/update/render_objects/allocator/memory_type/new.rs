use crate::update::render_objects::allocator::GpuMemoryType;
use alexandria::gpu::VulkanDevice;

impl GpuMemoryType {
    /// Create a new [`GpuMemoryType`]
    pub fn new(device: VulkanDevice, memory_type_index: u32) -> GpuMemoryType {
        GpuMemoryType {
            chunks: Vec::new(),
            memory_type_index,
            device,
        }
    }
}
