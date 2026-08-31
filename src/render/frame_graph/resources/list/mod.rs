use crate::render::frame_graph::FrameGraphTransientResource;
use alexandria::gpu::VulkanDeviceMemory;

mod index;
mod into_iter;
mod new;
mod resize;

/// A list of transient resources
pub(in crate::render::frame_graph::resources) struct FrameGraphResourceList {
    /// The transient resources that are at the render scale
    resources: Vec<FrameGraphTransientResource>,

    /// The memory used to hold the transient render scale resources
    memory: Option<VulkanDeviceMemory>,
}
