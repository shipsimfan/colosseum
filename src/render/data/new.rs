use crate::{
    Result,
    render::{FrameGraphNode, RenderData, RenderObjects, Skybox, data::DoubledRenderData},
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
        let mut descriptor_pool = render_objects.fixed().create_descriptor_pool(device)?;
        let doubled = [
            DoubledRenderData::new(
                &mut descriptor_pool,
                device,
                memory_properties,
                render_objects,
            )?,
            DoubledRenderData::new(
                &mut descriptor_pool,
                device,
                memory_properties,
                render_objects,
            )?,
        ];

        let mut descriptor_sets = Vec::new();
        FrameGraphNode::create_descriptor_sets(
            render_objects.fixed(),
            &mut descriptor_pool,
            &mut descriptor_sets,
        )?;

        Ok(RenderData {
            render_object_changes: Vec::new(),
            confirmed_removals: Vec::new(),
            descriptor_pool,
            post_process_descriptor_sets: descriptor_sets,

            render_scale: 1.0,
            gamma: 2.2,
            exposure: 1.0,
            contrast: 1.0,
            saturation: 1.0,

            skybox: Skybox::default(),

            doubled,
            current_doubled_index: 0,
        })
    }
}
