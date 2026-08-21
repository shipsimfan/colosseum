use crate::render::frame_graph::{
    FrameGraphDynamicTransientResourceInfo, FrameGraphResourceBuilder, FrameGraphResourceId,
};
use alexandria::gpu::VulkanFormat;

impl<'a> FrameGraphResourceBuilder<'a> {
    /// Create a new render scale transient resource and return its ID
    pub fn create_render_scale_transient(&mut self, format: VulkanFormat) -> FrameGraphResourceId {
        let id =
            FrameGraphResourceId::new_transient_render_scale(self.transient_render_scale.len());
        self.transient_render_scale
            .push(FrameGraphDynamicTransientResourceInfo::new(format));
        id
    }
}
