use crate::render::{MAX_FRAMES_IN_FLIGHT, frame_graph::FrameGraphTransientResourceInfo};
use alexandria::{
    gpu::{VulkanDeviceMemory, VulkanImageView},
    math::Vector2u,
};

mod data;

mod get;
mod new;
mod resize;

pub(in crate::render::frame_graph) use data::*;

/// A buffer of transient resources
pub(in crate::render::frame_graph) struct FrameGraphShadowMapBuffer {
    /// The name of this buffer
    memory_name: String,

    /// The information describing the resources
    info: FrameGraphTransientResourceInfo<ShadowMapData>,

    /// The size of the contained resources
    size: [Vector2u; MAX_FRAMES_IN_FLIGHT],

    /// The number of array layers allocated for the resources
    array_layers: [u32; MAX_FRAMES_IN_FLIGHT],

    /// The image views for the resources
    image_views: [Vec<VulkanImageView>; MAX_FRAMES_IN_FLIGHT],

    /// The image views of the full array layers for the resources
    full_image_view: [Option<VulkanImageView>; MAX_FRAMES_IN_FLIGHT],

    /// The memory associated with the resources
    memory: [Option<VulkanDeviceMemory>; MAX_FRAMES_IN_FLIGHT],
}
