use crate::render::frame_graph::{
    FrameGraphTransientResourceInfo, resources::FrameGraphTransientResourceBuffer,
};
use alexandria::{gpu::VulkanImageView, math::Vector2u};

impl FrameGraphTransientResourceBuffer {
    /// Get a resource based on its index
    pub fn get(
        &self,
        index: usize,
        frame_index: usize,
    ) -> (&FrameGraphTransientResourceInfo, &VulkanImageView, Vector2u) {
        (
            &self.info[index],
            &self.image_views[frame_index][index],
            self.size[frame_index],
        )
    }
}
