use crate::{
    Result,
    render::{
        FixedRenderObjects, ObjectData, PerFrameObjectBuilder, RenderCamera, RenderData,
        frame_graph::UnlitForwardRenderNode,
    },
};
use alexandria::gpu::{VulkanBufferUsageFlag, VulkanDescriptorType};

impl UnlitForwardRenderNode {
    /// Create needed per-frame resources for this node
    pub(in crate::render::frame_graph::nodes) fn create_per_frame_objects(
        per_frame_objects: &mut PerFrameObjectBuilder,
    ) -> Result<()> {
        // Create the camera descriptor set
        per_frame_objects.add_descriptor_set(
            FixedRenderObjects::CAMERA_DESCRIPTOR_SET_LAYOUT,
            FixedRenderObjects::CAMERA_DESCRIPTOR_SET,
        )?;

        // Create the camera device data buffer
        per_frame_objects.add_device_data_buffer::<RenderCamera, _>(
            1,
            VulkanBufferUsageFlag::UniformBuffer,
            FixedRenderObjects::CAMERA_DESCRIPTOR_SET,
            VulkanDescriptorType::UniformBuffer,
            0,
            FixedRenderObjects::CAMERA_DEVICE_BUFFER,
        )?;

        // Create the renderables descriptor set
        per_frame_objects.add_descriptor_set(
            FixedRenderObjects::RENDERABLES_DESCRIPTOR_SET_LAYOUT,
            FixedRenderObjects::RENDERABLES_DESCRIPTOR_SET,
        )?;

        // Create the renderables device data buffer
        per_frame_objects.add_device_data_buffer::<ObjectData, _>(
            RenderData::RENDERABLE_BUFFER_INIT_CAPACITY,
            VulkanBufferUsageFlag::StorageBuffer,
            FixedRenderObjects::RENDERABLES_DESCRIPTOR_SET,
            VulkanDescriptorType::StorageBuffer,
            0,
            FixedRenderObjects::RENDERABLES_DEVICE_BUFFER,
        )?;

        Ok(())
    }
}
