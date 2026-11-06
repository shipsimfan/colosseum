use crate::render::{
    FrameIndex,
    frame_graph::{
        FrameGraphTransientResourceInfo, ShadowMapData, resources::FrameGraphShadowMapBuffer,
    },
};
use alexandria::{gpu::VulkanImageView, math::Vector2u};

impl FrameGraphShadowMapBuffer {
    /// Get the size of the shadow map resource
    pub fn size(&self, frame_index: &FrameIndex) -> Vector2u {
        self.size[**frame_index]
    }

    /// Get the image view for a specific layer
    pub fn image_view(&self, frame_index: &FrameIndex, index: usize) -> &VulkanImageView {
        &self.image_views[**frame_index][index]
    }

    /// Get the full image view for a shadow map resource
    pub fn get_full(
        &self,
        frame_index: usize,
    ) -> (
        &FrameGraphTransientResourceInfo<ShadowMapData>,
        &VulkanImageView,
        Vector2u,
        u32,
    ) {
        (
            &self.info,
            &self.full_image_view[frame_index].as_ref().unwrap(),
            self.size[frame_index],
            self.array_layers[frame_index],
        )
    }
}
