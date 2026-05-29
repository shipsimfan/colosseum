use crate::render::job::GraphicsDevice;
use alexandria::gpu::VulkanSwapchain;

mod new;

/// The swapchain for the render job, which holds the graphics device and the Vulkan swapchain itself
pub(in crate::render::job) struct Swapchain<'surface> {
    /// The graphics device to use for rendering
    device: GraphicsDevice<'surface>,

    /// The swapchain itself
    swapchain: VulkanSwapchain<'surface>,
}
