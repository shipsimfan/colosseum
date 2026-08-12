use crate::{
    Result,
    logging::Logger,
    render::{CameraRenderData, RenderObjects, data::DoubledRenderData},
};
use alexandria::gpu::{VulkanAdapterMemoryProperties, VulkanDevice};

impl DoubledRenderData {
    /// Create a new set of [`DoubledRenderData`]
    pub fn new(
        device: &VulkanDevice,
        memory_properties: &VulkanAdapterMemoryProperties,
        render_objects: &RenderObjects,
        logger: &Logger,
    ) -> Result<DoubledRenderData> {
        let camera = CameraRenderData::new(device, memory_properties, render_objects, logger)?;

        Ok(DoubledRenderData { camera })
    }
}
