use crate::update::render_objects::allocator::GpuMemoryChunk;
use alexandria::gpu::VulkanDevice;

mod allocate;
mod new;
mod supports;

/// A set of chunks on a specific memory type that can be used to allocate GPU resources
pub(in crate::update::render_objects::allocator) struct GpuMemoryType {
    /// The chunks of GPU memory that have been allocated
    chunks: Vec<GpuMemoryChunk>,

    /// The memory type index that this memory type corresponds to
    memory_type_index: u32,

    /// The device to allocate GPU memory from
    device: VulkanDevice,
}
