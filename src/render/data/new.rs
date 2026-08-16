use crate::{
    Result,
    render::{RenderData, RenderObjects, Skybox, data::DoubledRenderData},
};
use alexandria::gpu::{VulkanAdapterMemoryProperties, VulkanDevice};
use std::sync::Arc;

impl RenderData {
    /// Create a new set of [`RenderData`]
    pub(in crate::render) fn new(
        device: &VulkanDevice,
        memory_properties: &Arc<VulkanAdapterMemoryProperties>,
        render_objects: &RenderObjects,
    ) -> Result<RenderData> {
        Ok(RenderData {
            render_object_changes: Vec::new(),
            confirmed_removals: Vec::new(),

            skybox: Skybox::default(),
            doubled: [
                DoubledRenderData::new(device, memory_properties, render_objects)?,
                DoubledRenderData::new(device, memory_properties, render_objects)?,
            ],
            current_doubled_index: 0,
        })
    }
}
