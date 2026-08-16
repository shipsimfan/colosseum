use crate::render::{Material, Mesh};
use alexandria::{
    Id,
    gpu::{GpuAddress, VulkanBuffer, VulkanMappedMemory},
};

mod iter;
mod new;
mod push;
mod reset;

/// A single chunk of allocated renderable data
pub(in crate::render::data::renderable_list) struct RenderableBuffer<T> {
    /// The GPU buffer containing the object data
    #[allow(unused)]
    buffer: VulkanBuffer,

    /// The mapped memory for writing object data
    memory: VulkanMappedMemory<T>,

    /// The base address of the buffer in GPU memory
    base_address: GpuAddress<T>,

    /// The list of renderable objects in this buffer
    renderables: Vec<(Id<Material>, Id<Mesh>, GpuAddress<T>)>,
}

impl<T> RenderableBuffer<T> {
    /// The maximum number of renderable objects that can be stored in a single buffer
    pub const SIZE: usize = 1024 * 1024 / std::mem::size_of::<T>(); // 1 MB

    /// The size, in bytes, of the buffer
    pub const SIZE_BYTES: usize = Self::SIZE * std::mem::size_of::<T>();
}
