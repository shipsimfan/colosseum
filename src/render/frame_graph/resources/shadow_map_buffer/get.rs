use crate::render::frame_graph::{
    FrameGraphTransientResourceInfo, resources::FrameGraphShadowMapBuffer,
};
use alexandria::{gpu::VulkanImageView, math::Vector2u};

impl FrameGraphShadowMapBuffer {
    /// Get a resource based on its index
    pub fn get(
        &self,
        index: usize,
        frame_index: usize,
    ) -> (
        &FrameGraphTransientResourceInfo,
        &VulkanImageView,
        u32,
        Vector2u,
    ) {
        (
            &self.info,
            &self.image_views[frame_index][index],
            self.array_layers[frame_index],
            self.size[frame_index],
        )
    }
}
