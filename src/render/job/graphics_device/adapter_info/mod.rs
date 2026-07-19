use alexandria::{
    MemorySize, Uuid,
    gpu::{VulkanAdapter, VulkanAdapterType, VulkanFormat},
};

mod deref;
mod eq;
mod get;
mod is_compatible;
mod ord;

/// Information about a Vulkan adapter that is compatible with the surface and suitable for rendering
pub(in crate::render::job::graphics_device) struct VulkanAdapterInfo<'instance> {
    /// The Vulkan adapter itself
    adapter: VulkanAdapter<'instance>,

    /// The name of the Vulkan adapter
    name: String,

    /// The UUID of the Vulkan adapter
    uuid: Uuid,

    /// The type of the Vulkan adapter
    r#type: VulkanAdapterType,

    /// The swapchain format to use with this adapter
    swapchain_format: VulkanFormat,

    /// The index of the graphics queue family to use with this adapter
    graphics_queue_family_index: u32,

    /// The index of the transfer queue family to use with this adapter
    transfer_queue_family_index: u32,

    /// The amount of device-local VRAM available on this adapter, in bytes
    device_local_vram: MemorySize,

    /// The memory index for staging buffers on this adapter
    staging_buffer_memory_index: usize,

    /// The memory index for device-local buffers on this adapter
    device_local_buffer_memory_index: usize,
}
