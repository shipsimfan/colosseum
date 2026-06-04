use alexandria::{
    gpu::{VulkanColorSpace, VulkanImageView, VulkanSwapchain},
    math::Vector2u,
};
use frame_data::FrameData;

mod frame_data;

mod new;
mod next_frame;
mod unwrap;

/// The swapchain for the render job, which holds the graphics device and the Vulkan swapchain itself
pub(in crate::render::job) struct Swapchain<'surface> {
    /// The swapchain itself
    swapchain: VulkanSwapchain<'surface>,

    /// The image views for the swapchain images
    image_views: Vec<VulkanImageView>,

    /// The frame data for each frame in flight
    frame_data: Vec<FrameData>,

    /// The index of the next frame to use
    frame_index: usize,

    /// The size of the swapchain images
    size: Vector2u,
}

impl<'surface> Swapchain<'surface> {
    /// The color space to use for the swapchain
    pub const COLOR_SPACE: VulkanColorSpace = VulkanColorSpace::SRGBNonlinearKhr;
}
