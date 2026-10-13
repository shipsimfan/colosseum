use crate::render::{DeviceDataBuffer, FixedRenderObjects, ShadowMapBuffer, ShadowQuality};
use alexandria::gpu::{
    VulkanAdapterMemoryProperties, VulkanDescriptorPool, VulkanDescriptorSet, VulkanDevice,
};

mod add;
mod get;
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

    /// The shadow map buffers for the frame
    shadow_map_buffers: &'a mut Vec<ShadowMapBuffer>,

    /// The device to create buffers with
    device: &'a VulkanDevice,

    /// The memory properties to use for buffer allocation
    memory_properties: &'a VulkanAdapterMemoryProperties,

    /// The quality level for shadows
    shadow_quality: ShadowQuality,
}
