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
pub(in crate::render::data::renderable_list) struct RenderableBuffer<T> {}

impl<T> RenderableBuffer<T> {
    /// The maximum number of renderable objects that can be stored in a single buffer
    pub const SIZE: usize = 1024 * 1024 / std::mem::size_of::<T>(); // 1 MB

    /// The size, in bytes, of the buffer
    pub const SIZE_BYTES: usize = Self::SIZE * std::mem::size_of::<T>();
}
