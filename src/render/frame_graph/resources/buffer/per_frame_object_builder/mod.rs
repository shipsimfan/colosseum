use crate::render::{DeviceDataBuffer, FixedRenderObjects};
use alexandria::gpu::{VulkanDescriptorPool, VulkanDescriptorSet};

mod add;
mod new;

/// A builder for per-frame objects used by the frame graph transient buffer
pub(in crate::render) struct PerFrameObjectBuilder<'a> {
    /// The fixed render objects to use for creation
    fixed_render_objects: &'a FixedRenderObjects,

    /// The pool to allocate descriptor sets from
    descriptor_pool: &'a mut VulkanDescriptorPool,

    /// The currently allocated descriptor sets
    descriptor_sets: &'a mut Vec<VulkanDescriptorSet>,

    /// The set of device-local data buffers needed
    device_buffers: &'a mut Vec<DeviceDataBuffer>,
}
