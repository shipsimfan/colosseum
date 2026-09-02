use crate::render::frame_graph::resources::FrameGraphResourceList;
use alexandria::gpu::{VulkanDescriptorPool, VulkanDescriptorSet};

mod device_buffer;
mod per_frame_object_builder;

mod new;

pub(in crate::render) use device_buffer::*;
pub(in crate::render) use per_frame_object_builder::*;

/// A buffer for transient resources used by the frame graph in a specific frame
pub(in crate::render) struct FrameGraphTransientBuffer {
    /// The epoch this buffer is setup for
    pub(in crate::render::frame_graph::resources) epoch: u64,

    /// The transient resources that are at the render scale
    pub(in crate::render::frame_graph::resources) render_scale: FrameGraphResourceList,

    /// The transient resources that are at the native scale
    pub(in crate::render::frame_graph::resources) native_scale: FrameGraphResourceList,

    /// The pool for the descriptor sets
    #[allow(unused)]
    descriptor_pool: VulkanDescriptorPool,

    /// The descriptor sets that have been made for the frame
    pub descriptor_sets: Vec<VulkanDescriptorSet>,

    /// The device local data buffers made for the frame
    pub device_buffers: Vec<DeviceDataBuffer>,
}
