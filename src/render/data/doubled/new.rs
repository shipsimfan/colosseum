use crate::{
    Result,
    render::{
        CameraRenderData, RenderObjects,
        data::{DoubledRenderData, doubled::DataBuffer},
    },
};
use alexandria::gpu::{VulkanAdapterMemoryProperties, VulkanDescriptorPool, VulkanDevice};
use std::sync::Arc;

/// The initial capacity for the object data buffer
const OBJECT_BUFFER_INIT_CAPACITY: usize = 256;

impl DoubledRenderData {
    /// Create a new set of [`DoubledRenderData`]
    pub fn new(
        descriptor_pool: &mut VulkanDescriptorPool,
        device: &VulkanDevice,
        memory_properties: &Arc<VulkanAdapterMemoryProperties>,
        render_objects: &RenderObjects,
    ) -> Result<DoubledRenderData> {
        let camera =
            CameraRenderData::new(descriptor_pool, device, memory_properties, render_objects)?;
        let object_buffer =
            DataBuffer::new(OBJECT_BUFFER_INIT_CAPACITY, device, memory_properties)?;

        Ok(DoubledRenderData {
            camera,
            unlit_opaque_renderables: Vec::new(),
            object_buffer,
        })
    }
}
