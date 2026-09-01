use crate::render::{DeviceDataBuffer, FixedRenderObjects, PerFrameObjectBuilder};
use alexandria::gpu::{VulkanDescriptorPool, VulkanDescriptorSet};

impl<'a> PerFrameObjectBuilder<'a> {
    /// Create a new [`PerFrameObjectBuilder`]
    pub(in crate::render::frame_graph::resources::buffer) fn new(
        fixed_render_objects: &'a FixedRenderObjects,
        descriptor_pool: &'a mut VulkanDescriptorPool,
        descriptor_sets: &'a mut Vec<VulkanDescriptorSet>,
        device_buffers: &'a mut Vec<DeviceDataBuffer>,
    ) -> PerFrameObjectBuilder<'a> {
        PerFrameObjectBuilder {
            fixed_render_objects,
            descriptor_pool,
            descriptor_sets,
            device_buffers,
        }
    }
}
