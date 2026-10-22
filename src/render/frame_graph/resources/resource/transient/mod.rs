use crate::render::{MAX_FRAMES_IN_FLIGHT, frame_graph::FrameGraphResourceState};
use alexandria::gpu::{VulkanAttachmentLoadOp, VulkanFormat};
use std::ffi::CString;

mod create_image;
mod create_image_view;
mod get;
mod new;
mod set;
mod transition;

/// The information describing a transient resource
pub(in crate::render::frame_graph) struct FrameGraphTransientResourceInfo {
    /// The name of the transient resource
    name: &'static str,

    /// The C name for the image
    image_name: [CString; MAX_FRAMES_IN_FLIGHT],

    /// The C name for the image view
    image_view_name: [CString; MAX_FRAMES_IN_FLIGHT],

    /// The format of the resource
    format: VulkanFormat,

    /// The current state of the resource
    state: FrameGraphResourceState,

    /// The load operation for the resource
    load_op: VulkanAttachmentLoadOp,

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
}
