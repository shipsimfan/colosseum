use crate::render::frame_graph::Arena;
use alexandria::gpu::VulkanDeviceMemory;

mod buffer;
mod builder;
mod id;
mod resource;

mod get;
mod needs_resize;
mod new;
mod resize;

pub(in crate::render::frame_graph) use builder::*;
pub(in crate::render::frame_graph) use id::*;
pub(in crate::render::frame_graph) use resource::*;

pub(in crate::render) use buffer::*;

/// The resources that are available to the frame graph, which can be accessed by nodes during
/// execution
pub(in crate::render::frame_graph) struct FrameGraphResources<'a> {
    /// The external resources that are available to the frame graph, such as the swapchain image
    external: Arena<'a, FrameGraphExternalResource<'a>>,

    /// The epoch of the transient resources
    epoch: &'a mut u64,

    /// The transient resources that are at the render scale
    transient_render_scale: &'a mut Vec<FrameGraphTransientResource>,

    /// The memory used to hold the transient render scale resources
    transient_render_scale_memory: &'a mut Option<VulkanDeviceMemory>,
}
