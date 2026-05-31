use crate::render::job::GraphicsDevice;
use alexandria::{
    gpu::{VulkanImageView, VulkanSwapchain},
    math::Vector2u,
};
use frame_data::FrameData;

mod frame_context;
mod frame_data;

mod new;
mod next_frame;
mod unwrap;

pub(in crate::render) use frame_context::FrameContext;

/// The swapchain for the render job, which holds the graphics device and the Vulkan swapchain itself
pub(in crate::render::job) struct Swapchain<'surface> {
    /// The graphics device to use for rendering
    device: GraphicsDevice<'surface>,

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
