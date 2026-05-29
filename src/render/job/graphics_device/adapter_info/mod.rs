use alexandria::gpu::{VulkanAdapter, VulkanFormat};

mod deref;
mod eq;
mod get;
mod is_compatible;
mod ord;

/// Information about a Vulkan adapter that is compatible with the surface and suitable for rendering
pub(in crate::render::job::graphics_device) struct VulkanAdapterInfo<'instance> {
    /// The Vulkan adapter itself
    adapter: VulkanAdapter<'instance>,

    /// The swapchain format to use with this adapter
    swapchain_format: VulkanFormat,

    /// The index of the graphics queue family to use with this adapter
    graphics_queue_family_index: u32,
}
