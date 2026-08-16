use alexandria::gpu::{VulkanAdapterMemoryProperties, VulkanDevice};
use buffer::RenderableBuffer;
use std::sync::Arc;

mod buffer;

mod iter;
mod new;
mod push;
mod reset;

/// The list of renderable objects in the scene
pub(in crate::render::data) struct RenderableList<T> {
    /// The set of allocated buffers for renderable objects
    buffers: Vec<RenderableBuffer<T>>,

    /// The current total number of renderable objects in the list
    count: usize,

    /// The device to use for allocating new buffers
    device: VulkanDevice,

    /// The memory properties to use for allocating new buffers
    memory_properties: Arc<VulkanAdapterMemoryProperties>,
}
