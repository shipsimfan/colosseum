use crate::render::{MAX_FRAMES_IN_FLIGHT, frame_graph::FrameGraphTransientResourceInfo};
use alexandria::{
    gpu::{VulkanDeviceMemory, VulkanImageView},
    math::Vector2u,
};

mod get;
mod new;
mod resize;

/// A buffer of transient resources
pub(in crate::render::frame_graph::resources) struct FrameGraphTransientResourceBuffer {
    /// The name of this buffer
    memory_name: &'static str,

    /// The information describing the resources
    info: Vec<FrameGraphTransientResourceInfo>,

    /// The size of the contained resources
    size: [Vector2u; MAX_FRAMES_IN_FLIGHT],

    /// The image views for the resources
    image_views: [Vec<VulkanImageView>; MAX_FRAMES_IN_FLIGHT],

    /// The memory associated with the resources
    memory: [Option<VulkanDeviceMemory>; MAX_FRAMES_IN_FLIGHT],
}
