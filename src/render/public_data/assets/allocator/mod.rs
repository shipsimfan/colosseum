use chunk::*;
use memory_type::*;
use std::sync::Arc;

mod allocated_memory;
mod chunk;
mod memory_type;

mod allocate;
mod free;
mod new;

pub(in crate::render) use allocated_memory::*;

use crate::render::GraphicsDevice;

/// An allocator for Vulkan resources on the GPU
pub(in crate::render::public_data::assets) struct GpuAllocator {
    /// The name of the allocator
    name: &'static str,

    /// The memory types that can be used to allocate GPU resources
    memory_types: Vec<GpuMemoryType>,

    /// The size of each chunk of GPU memory
    chunk_size: u32,

    /// The minimum size of a block of GPU memory that can be allocated
    min_block_size: u32,

    /// The maximum size of a block of GPU memory that can be allocated
    max_block_size: u32,

    /// The device to allocate GPU memory from
    device: Arc<GraphicsDevice>,
}
