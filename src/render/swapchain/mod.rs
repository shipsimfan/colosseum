use alexandria::{
    gpu::{VulkanColorSpace, VulkanDevice, VulkanImageView, VulkanSwapchain},
    math::Vector2u,
};
use colosseum_core::Logger;

mod drop;
mod new;
mod render_frame;

/// The swapchain for the render job, which holds the graphics device and the Vulkan swapchain itself
pub(in crate::render) struct Swapchain {
    /// The logger for operations related to the graphics device
    logger: Logger,

    /// The swapchain itself
    swapchain: VulkanSwapchain,

    /// The image views for the swapchain images
    image_views: Vec<VulkanImageView>,

    /// The size of the swapchain images
    size: Vector2u,

    /// The device the swapchain was created with
    device: VulkanDevice,
}

impl Swapchain {
    /// The color space to use for the swapchain
    pub const COLOR_SPACE: VulkanColorSpace = VulkanColorSpace::SRGBNonlinearKhr;
}
