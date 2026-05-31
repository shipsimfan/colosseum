use crate::render::job::swapchain::FrameData;
use alexandria::{
    gpu::{VulkanImageView, VulkanQueue, VulkanSwapchain},
    math::Vector2u,
};

mod begin_rendering_swapchain;
mod deref;
mod new;
mod present;

/// The data required to render a single frame
#[must_use]
pub(in crate::render) struct FrameContext<'frame, 'surface> {
    /// The per-frame objects
    data: &'frame mut FrameData,

    /// The queue to submit the commands to
    queue: &'frame mut VulkanQueue,

    /// The index of the current image in the swapchain
    image_index: u32,

    /// The image view for the current frame
    image_view: &'frame mut VulkanImageView,

    /// The swapchain to present to
    swapchain: &'frame mut VulkanSwapchain<'surface>,

    /// The size of the swapchain images
    size: Vector2u,
}
