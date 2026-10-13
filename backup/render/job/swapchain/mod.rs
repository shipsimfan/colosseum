use alexandria::{
    gpu::{VulkanColorSpace, VulkanCommandPool, VulkanDevice, VulkanImageView, VulkanSwapchain},
    math::Vector2u,
};
use frame_data::FrameData;

mod frame_data;

mod drop;
mod new;
mod next_frame;
mod unwrap;

/// The swapchain for the render job, which holds the graphics device and the Vulkan swapchain itself
pub(in crate::render::job) struct Swapchain<'surface> {
    /// The swapchain itself
    swapchain: Option<VulkanSwapchain<'surface>>,

    /// The image views for the swapchain images
    image_views: Vec<VulkanImageView>,

    /// The command pool for the graphics queue
    command_pool: VulkanCommandPool,

    /// The frame data for each frame in flight
    frame_data: Vec<FrameData>,

    /// The index of the next frame to use
    frame_index: usize,

    /// The size of the swapchain images
    size: Vector2u,

    /// The device the swapchain was created with
    device: VulkanDevice,
}

impl<'surface> Swapchain<'surface> {
    /// The color space to use for the swapchain
    pub const COLOR_SPACE: VulkanColorSpace = VulkanColorSpace::SRGBNonlinearKhr;
}
