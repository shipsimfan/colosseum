use crate::logging::Logger;
use adapter_info::VulkanAdapterInfo;
use alexandria::gpu::{VulkanCommandPool, VulkanDevice, VulkanFormat, VulkanSurface};

mod adapter_info;

mod allocate_command_buffer;
mod deref;
mod drop;
mod get;
mod get_adapters;
mod new;

/// The graphics device is responsible for managing the Vulkan device and related resources
pub(in crate::render::job) struct GraphicsDevice<'surface> {
    /// The logger for operations related to the graphics device
    logger: Logger,

    /// The Vulkan device
    device: VulkanDevice,

    /// The command pool for the graphics queue
    command_pool: VulkanCommandPool,

    /// The surface to use for the swapchain
    surface: &'surface VulkanSurface,

    /// The format of the swapchain images, which is determined when creating the swapchain
    swapchain_format: VulkanFormat,
}
