use crate::{
    Result,
    render::{
        FixedRenderObjects, FrameGraphNode, FrameGraphTransientBuffer,
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
        FrameGraphNode::create_descriptor_sets(
            render_objects,
            &mut descriptor_pool,
            &mut descriptor_sets,
        )?;

        Ok(FrameGraphTransientBuffer {
            epoch: 0,
            render_scale: FrameGraphResourceList::new(),
            native_scale: FrameGraphResourceList::new(),

            descriptor_pool,
            descriptor_sets,
        })
    }
}
