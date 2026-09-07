use crate::{
    Result,
    render::{
        FixedRenderObjects, ObjectData, PerFrameObjectBuilder, RenderData,
        frame_graph::UnlitForwardRenderNode,
    },
};
use alexandria::gpu::{VulkanBufferUsageFlag, VulkanDescriptorType};

impl UnlitForwardRenderNode {
    /// Create needed per-frame resources for this node
    pub(in crate::render::frame_graph::nodes) fn create_per_frame_objects(
        per_frame_objects: &mut PerFrameObjectBuilder,
    ) -> Result<()> {
        // Create the renderables descriptor set
        per_frame_objects.add_descriptor_set(
            FixedRenderObjects::RENDERABLES_DESCRIPTOR_SET_LAYOUT,
            FixedRenderObjects::RENDERABLES_DESCRIPTOR_SET,
        )?;

        // Create the renderables device data buffer
        per_frame_objects.add_device_data_buffer::<ObjectData, _>(
            RenderData::RENDERABLE_BUFFER_INIT_CAPACITY,
            VulkanBufferUsageFlag::StorageBuffer,
            VulkanDescriptorType::StorageBuffer,
            vec![(0, FixedRenderObjects::RENDERABLES_DESCRIPTOR_SET).into()],
            FixedRenderObjects::RENDERABLES_DEVICE_BUFFER,
        )?;

        Ok(())
    }
}
