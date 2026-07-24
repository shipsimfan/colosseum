use alexandria::gpu::{VulkanAdapterMemoryProperties, VulkanDevice};
use chunk::GpuMemoryChunk;
use memory_type::GpuMemoryType;
use std::sync::Arc;

mod allocated_memory;
mod chunk;
mod memory_type;

mod allocate;
mod new;

pub(crate) use allocated_memory::*;

/// An allocator for Vulkan resources on the GPU
pub(in crate::update::render_objects) struct GpuAllocator {
    /// The memory types that can be used to allocate GPU resources
    memory_types: Vec<GpuMemoryType>,

    /// The size of each chunk of GPU memory
    chunk_size: u32,

    /// The minimum size of a block of GPU memory that can be allocated
    min_block_size: u32,

    /// The maximum size of a block of GPU memory that can be allocated
    max_block_size: u32,

    /// The memory properties of the Vulkan adapter that this allocator is using
    memory_properties: Arc<VulkanAdapterMemoryProperties>,

    /// The device to allocate GPU memory from
    device: VulkanDevice,
}
