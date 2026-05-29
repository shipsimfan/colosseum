use crate::render::job::GraphicsDevice;
use alexandria::gpu::{VulkanImageView, VulkanSwapchain};

mod frame_context;
mod new;

pub(in crate::render) use frame_context::FrameContext;

/// The swapchain for the render job, which holds the graphics device and the Vulkan swapchain itself
pub(in crate::render::job) struct Swapchain<'surface> {
    /// The graphics device to use for rendering
    device: GraphicsDevice<'surface>,

    /// The swapchain itself
    swapchain: VulkanSwapchain<'surface>,

    /// The image views for the swapchain images
    image_views: Vec<VulkanImageView>,

    /// The frame contexts for each frame in flight
    frame_contexts: Vec<FrameContext>,
}
