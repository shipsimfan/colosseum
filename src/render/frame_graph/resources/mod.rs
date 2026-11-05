use crate::render::{DescriptorSetBinding, MAX_FRAMES_IN_FLIGHT};
use alexandria::gpu::{VulkanDescriptorPool, VulkanDescriptorSet};
use buffer::*;
use data_binding::*;
use shadow_map_buffer::*;

mod buffer;
mod data_binding;
mod id;
mod resource;
mod shadow_map_buffer;
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

    /// The shadow map buffer
    shadow_map: FrameGraphTransientResourceBuffer,

    /// The descriptor pool containing all the descriptor sets used within the frame graph
    #[allow(unused)]
    descriptor_pool: Option<VulkanDescriptorPool>,

    /// The collection of all descriptor sets used within the frame graph
    descriptor_sets: Vec<(
        [VulkanDescriptorSet; MAX_FRAMES_IN_FLIGHT],
        &'static [DescriptorSetBinding],
    )>,

    /// The bindings of the data buffers to descriptor sets
    data_bindings: Vec<FrameGraphDataBinding>,
}
