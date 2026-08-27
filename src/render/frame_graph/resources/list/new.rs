use crate::render::frame_graph::{FrameGraphTransientResource, resources::FrameGraphResourceList};
use alexandria::gpu::VulkanDeviceMemory;

impl<'a> FrameGraphResourceList<'a> {
    /// Create a new [`FrameGraphResourceList`]
    pub fn new(
        resources: &'a mut Vec<FrameGraphTransientResource>,
        memory: &'a mut Option<VulkanDeviceMemory>,
    ) -> FrameGraphResourceList<'a> {
        FrameGraphResourceList { resources, memory }
    }
}
