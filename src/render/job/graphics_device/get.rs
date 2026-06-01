use crate::{
    logging::Logger,
    render::{FrameGraph, job::GraphicsDevice},
};
use alexandria::gpu::{VulkanFormat, VulkanQueue, VulkanSurface};

impl<'surface> GraphicsDevice<'surface> {
    /// Get the logger for this graphics device
    pub fn logger(&self) -> &Logger {
        &self.logger
    }

    /// Get the surface that this graphics device is rendering to
    pub fn surface(&self) -> &'surface VulkanSurface {
        self.surface
    }

    /// Get the format for swapchain images that this graphics device supports
    pub fn swapchain_format(&self) -> VulkanFormat {
        self.swapchain_format
    }

    /// Get the queue that this graphics device uses for rendering
    pub fn queue_and_frame_graph(&mut self) -> (&mut VulkanQueue, &mut FrameGraph) {
        (&mut self.queue, &mut self.frame_graph)
    }
}
