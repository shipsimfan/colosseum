use crate::update::render_objects::GpuAllocator;
use alexandria::gpu::{VulkanAdapterMemoryProperties, VulkanDevice};
use std::sync::Arc;

impl GpuAllocator {
    /// Create a new [`GpuAllocator`]
    pub fn new(
        chunk_size: u32,
        min_block_size: u32,
        max_block_size: u32,
        memory_properties: Arc<VulkanAdapterMemoryProperties>,
        device: VulkanDevice,
    ) -> GpuAllocator {
        assert!(
            chunk_size.is_power_of_two(),
            "chunk_size must be a power of two"
        );
        assert!(
            min_block_size.is_power_of_two(),
            "min_block_size must be a power of two"
        );
        assert!(
            max_block_size.is_power_of_two(),
            "max_block_size must be a power of two"
        );
        assert!(
            min_block_size <= max_block_size,
            "min_block_size must be less than or equal to max_block_size"
        );
        assert!(
            chunk_size >= max_block_size,
            "chunk_size must be greater than or equal to max_block_size"
        );

        GpuAllocator {
            memory_types: Vec::new(),
            chunk_size,
            min_block_size,
            max_block_size,
            memory_properties,
            device,
        }
    }
}
