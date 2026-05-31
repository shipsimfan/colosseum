use crate::render::{FrameContext, job::swapchain::FrameData};
use alexandria::{
    gpu::{VulkanImageView, VulkanQueue, VulkanSwapchain},
    math::Vector2u,
};

impl<'frame, 'surface> FrameContext<'frame, 'surface> {
    /// Create a new [`FrameContext`]
    pub(in crate::render::job::swapchain) fn new(
        data: &'frame mut FrameData,
        queue: &'frame mut VulkanQueue,
        image_index: u32,
        image_view: &'frame mut VulkanImageView,
        swapchain: &'frame mut VulkanSwapchain<'surface>,
        size: Vector2u,
    ) -> FrameContext<'frame, 'surface> {
        FrameContext {
            data,
            queue,
            image_index,
            image_view,
            swapchain,
            size,
        }
    }
}
