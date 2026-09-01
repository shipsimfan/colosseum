use crate::{
    Result,
    render::{
        FixedRenderObjects, FrameGraphNode, FrameGraphTransientBuffer, PerFrameObjectBuilder,
        frame_graph::resources::FrameGraphResourceList,
    },
};
use alexandria::gpu::VulkanDevice;

impl FrameGraphTransientBuffer {
    /// Create a new [`FrameGraphTransientBuffer`]
    pub fn new(
        render_objects: &FixedRenderObjects,
        device: &VulkanDevice,
    ) -> Result<FrameGraphTransientBuffer> {
        let mut descriptor_pool = render_objects.create_descriptor_pool(device)?;
        let mut descriptor_sets = Vec::new();
        let mut device_buffers = Vec::new();
        FrameGraphNode::create_per_frame_objects(PerFrameObjectBuilder::new(
            render_objects,
            &mut descriptor_pool,
            &mut descriptor_sets,
            &mut device_buffers,
        ))?;

        Ok(FrameGraphTransientBuffer {
            epoch: 0,
            render_scale: FrameGraphResourceList::new(),
            native_scale: FrameGraphResourceList::new(),

            descriptor_pool,
            descriptor_sets,
            device_buffers,
        })
    }
}
