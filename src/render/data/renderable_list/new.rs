use crate::{
    Result,
    render::data::{RenderableList, renderable_list::RenderableBuffer},
};
use alexandria::gpu::{VulkanAdapterMemoryProperties, VulkanDevice};
use std::sync::Arc;

impl<T> RenderableList<T> {
    /// Create a new [`RenderableList`]
    pub fn new(
        device: &VulkanDevice,
        memory_properties: &Arc<VulkanAdapterMemoryProperties>,
    ) -> Result<RenderableList<T>> {
        Ok(RenderableList {
            buffers: vec![RenderableBuffer::new(device, memory_properties)?],
            count: 0,
            device: device.clone(),
            memory_properties: memory_properties.clone(),
        })
    }
}
