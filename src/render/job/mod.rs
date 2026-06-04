use alexandria::gpu::VulkanSurface;
use graphics_device::GraphicsDevice;
use swapchain::Swapchain;

mod graphics_device;
mod swapchain;

mod new;
mod run;

/// The persistent state of the render job
#[allow(private_interfaces)]
pub(crate) enum RenderJob<'surface> {
    /// The render job is currently rendering
    Rendering {
        /// The device to use for rendering
        device: GraphicsDevice,

        /// The current swapchain, if it doesn't need to be recreated
        swapchain: Swapchain<'surface>,
    },

    /// The swapchain needs to be recreated
    RecreateSwapchain {
        /// The device to use for rendering
        device: GraphicsDevice,

        /// The surface to use for the swapchain
        surface: &'surface mut VulkanSurface,
    },
}
