use crate::render::frame_graph::{
    FrameGraphBuilder, FrameGraphResourceId, FrameGraphResourceKind,
    FrameGraphTransientResourceInfo,
};
use alexandria::gpu::VulkanFormat;

impl<'a> FrameGraphBuilder<'a> {
    /// Create a new transient resource
    pub fn create_resource(
        &mut self,
        name: &'static str,
        format: VulkanFormat,
        kind: FrameGraphResourceKind,
    ) -> FrameGraphResourceId {
        match kind {
            FrameGraphResourceKind::Native => {
                self.native_scale_resources
                    .push(FrameGraphTransientResourceInfo::new(name, format));
                FrameGraphResourceId::new_transient_native_scale(
                    self.native_scale_resources.len() - 1,
                )
            }
        }
    }
}
