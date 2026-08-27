use crate::render::frame_graph::FrameGraphTransientResource;
use alexandria::gpu::VulkanDeviceMemory;

mod index;
mod new;
mod resize;

/// A list of transient resources
pub(in crate::render::frame_graph::resources) struct FrameGraphResourceList<'a> {
    /// The transient resources that are at the render scale
    resources: &'a mut Vec<FrameGraphTransientResource>,

    /// The memory used to hold the transient render scale resources
    memory: &'a mut Option<VulkanDeviceMemory>,
}
