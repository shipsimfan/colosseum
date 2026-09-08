use crate::{
    Result,
    render::{
        FixedRenderObjects, FrameGraphNode, FrameGraphTransientBuffer, PerFrameObjectBuilder,
        frame_graph::resources::FrameGraphResourceList,
    },
};
use alexandria::gpu::{VulkanAdapterMemoryProperties, VulkanDevice};

impl FrameGraphTransientBuffer {
    /// Create a new [`FrameGraphTransientBuffer`]
    pub fn new(
        index: usize,
        render_objects: &FixedRenderObjects,
        device: &VulkanDevice,
        memory_properties: &VulkanAdapterMemoryProperties,
    ) -> Result<FrameGraphTransientBuffer> {
        let mut descriptor_pool = render_objects.create_descriptor_pool(index, device)?;
        let mut descriptor_sets = Vec::new();
        let mut device_buffers = Vec::new();
        let mut shadow_map_buffers = Vec::new();
        FrameGraphNode::create_per_frame_objects(
            PerFrameObjectBuilder::new(
                render_objects,
                &mut descriptor_pool,
                &mut descriptor_sets,
                &mut device_buffers,
                &mut shadow_map_buffers,
                device,
                memory_properties,
            ),
            index,
        )?;

        Ok(FrameGraphTransientBuffer {
            epoch: 0,
            render_scale: FrameGraphResourceList::new(),
            native_scale: FrameGraphResourceList::new(),

            descriptor_pool,
            descriptor_sets,
            device_buffers,
            shadow_map_buffers,
        })
    }
}
