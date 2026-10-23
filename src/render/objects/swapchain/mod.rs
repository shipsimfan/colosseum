use alexandria::{
    gpu::{VulkanColorSpace, VulkanImageView, VulkanSwapchain},
    math::Vector2u,
};
use colosseum_core::Logger;
use std::sync::Arc;

use crate::render::GraphicsDevice;

mod acquire;
mod drop;
mod get;
mod new;
mod present;

/// The swapchain for the render job, which holds the graphics device and the Vulkan swapchain itself
pub(in crate::render) struct Swapchain {
    /// The logger for operations related to the graphics device
    logger: Logger,

    /// The swapchain itself
    swapchain: VulkanSwapchain,

    /// The image views for the swapchain images
    image_views: Vec<Arc<VulkanImageView>>,

    /// The size of the swapchain images
    size: Vector2u,

    /// The device the swapchain was created with
    device: Arc<GraphicsDevice>,

    /// The image index of the currently acquired swapchain image
    image_index: usize,

    /// Is the swapchain currently valid?
    valid: bool,

    /// Is the swapchain currently suboptimal?
    suboptimal: bool,
}

impl Swapchain {
    /// The color space to use for the swapchain
    pub const COLOR_SPACE: VulkanColorSpace = VulkanColorSpace::SRGBNonlinearKhr;
}
