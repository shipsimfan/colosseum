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

    /// Is this resource a transfer destination resource?
    is_transfer_dst: bool,

    /// Is this resource a transfer source resource?
    is_transfer_src: bool,

    /// Is this resource a sampled image resource?
    is_sampled_image: bool,

    /// The current state of the resource
    state: FrameGraphResourceState,
}
