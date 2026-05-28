use crate::logging::Logger;
use alexandria::gpu::{VulkanCommandPool, VulkanDevice};

mod get_adapters;
mod new;

/// The graphics device is responsible for managing the Vulkan device and related resources
pub(in crate::run::game) struct GraphicsDevice {
    /// The logger for operations related to the graphics device
    logger: Logger,

    /// The Vulkan device
    device: VulkanDevice,

    /// The command pool for the graphics queue
    command_pool: VulkanCommandPool,
}
