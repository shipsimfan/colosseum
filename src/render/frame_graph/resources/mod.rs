use crate::render::{DescriptorSetBinding, MAX_FRAMES_IN_FLIGHT};
use alexandria::gpu::{VulkanDescriptorPool, VulkanDescriptorSet};
use buffer::*;

mod buffer;
mod id;
mod resource;
mod state;
mod usage;

mod clear;
mod get;
mod insert;
mod new;
mod resize;

pub(in crate::render::frame_graph) use id::*;
pub(in crate::render::frame_graph) use resource::*;
pub(in crate::render::frame_graph) use state::*;
pub(in crate::render::frame_graph) use usage::*;

/// The collection of all resources used within the frame graph
pub(in crate::render::frame_graph) struct FrameGraphResources {
    /// The external resources used within the frame graph
    external_resources: Vec<FrameGraphExternalResource>,

    /// The transient native scale resources used within the frame graph
    native_scale: FrameGraphTransientResourceBuffer,

    /// The transient render scale resources used within the frame graph
    render_scale: FrameGraphTransientResourceBuffer,

    /// The descriptor pool containing all the descriptor sets used within the frame graph
    #[allow(unused)]
    descriptor_pool: Option<VulkanDescriptorPool>,

    /// The collection of all descriptor sets used within the frame graph
    descriptor_sets: Vec<(
        [VulkanDescriptorSet; MAX_FRAMES_IN_FLIGHT],
        &'static [DescriptorSetBinding],
    )>,
}
