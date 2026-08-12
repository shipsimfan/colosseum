use crate::{
    Result,
    logging::Logger,
    render::{RenderData, RenderObjects, Skybox, data::DoubledRenderData},
};
use alexandria::gpu::{VulkanAdapterMemoryProperties, VulkanDevice};

impl RenderData {
    /// Create a new set of [`RenderData`]
    pub(in crate::render) fn new(
        device: &VulkanDevice,
        memory_properties: &VulkanAdapterMemoryProperties,
        render_objects: &RenderObjects,
        logger: &Logger,
    ) -> Result<RenderData> {
        Ok(RenderData {
            render_object_changes: Vec::new(),
            confirmed_removals: Vec::new(),

            skybox: Skybox::default(),
            unlit_opaque_renderables: Vec::new(),
            doubled: [
                DoubledRenderData::new(device, memory_properties, render_objects, logger)?,
                DoubledRenderData::new(device, memory_properties, render_objects, logger)?,
            ],
            current_doubled_index: 0,
        })
    }
}
