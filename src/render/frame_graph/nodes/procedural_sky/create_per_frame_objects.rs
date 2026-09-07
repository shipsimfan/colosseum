use crate::{
    Result,
    render::{
        FixedRenderObjects, PerFrameObjectBuilder, RenderCamera, frame_graph::ProceduralSkyNode,
    },
};
use alexandria::gpu::{VulkanBufferUsageFlag, VulkanDescriptorType};

impl ProceduralSkyNode {
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
            VulkanDescriptorType::UniformBuffer,
            vec![(0, FixedRenderObjects::CAMERA_DESCRIPTOR_SET).into()],
            FixedRenderObjects::CAMERA_DEVICE_BUFFER,
        )
    }
}
