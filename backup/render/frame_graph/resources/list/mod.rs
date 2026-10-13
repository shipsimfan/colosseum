use crate::render::frame_graph::FrameGraphTransientResource;
use alexandria::gpu::VulkanDeviceMemory;
use std::ffi::CString;

mod index;
mod into_iter;
mod new;
mod resize;

/// A list of transient resources
pub(in crate::render::frame_graph::resources) struct FrameGraphResourceList {
    /// The name for the resource list
    name: CString,

    /// The transient resources that are in the list
    resources: Vec<FrameGraphTransientResource>,

    /// The memory used to hold the transient resources
    memory: Option<VulkanDeviceMemory>,
}
