use crate::{
    Result,
    render::{
        CameraRenderData, RenderObjects,
        data::{DoubledRenderData, RenderableList},
    },
};
use alexandria::gpu::{VulkanAdapterMemoryProperties, VulkanDescriptorPool, VulkanDevice};
use std::sync::Arc;

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
        let unlit_opaque_renderables = RenderableList::new(device, memory_properties)?;

        Ok(DoubledRenderData {
            camera,
            unlit_opaque_renderables,
        })
    }
}
