use crate::update::render_objects::allocator::GpuMemoryType;
use alexandria::gpu::VulkanMemoryRequirements;

impl GpuMemoryType {
    /// Does this memory type support the given memory requirements?
    pub fn supports(&self, memory_requirements: &VulkanMemoryRequirements) -> bool {
        memory_requirements.memory_type_bits() & (1 << self.memory_type_index) != 0
    }
}
