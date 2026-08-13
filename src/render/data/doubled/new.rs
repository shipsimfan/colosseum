use crate::{
    Result,
    render::{CameraRenderData, RenderObjects, data::DoubledRenderData},
};
use alexandria::gpu::{VulkanAdapterMemoryProperties, VulkanDevice};

impl DoubledRenderData {
    /// Create a new set of [`DoubledRenderData`]
    pub fn new(
        device: &VulkanDevice,
        memory_properties: &VulkanAdapterMemoryProperties,
        render_objects: &RenderObjects,
    ) -> Result<DoubledRenderData> {
        let camera = CameraRenderData::new(device, memory_properties, render_objects)?;

        Ok(DoubledRenderData { camera })
    }
}
