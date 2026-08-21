use crate::render::frame_graph::FrameGraphResourceState;
use alexandria::gpu::VulkanFormat;

mod get;
mod new;
mod set;
mod transition;

/// The information describing a transient resource which has a size determined externally at runtime
pub(in crate::render::frame_graph) struct FrameGraphDynamicTransientResourceInfo {
    /// The format of the resource
    format: VulkanFormat,

    /// Is this resource a color resource?
    is_color: bool,

    /// Is this resource a depth resource?
    is_depth: bool,

    /// The current state of the resource
    state: FrameGraphResourceState,
}
