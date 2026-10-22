use crate::render::frame_graph::{
    FrameGraphResourceId, FrameGraphResourceKind, FrameGraphTransientResourceInfo,
    builder::FrameGraphBuilderResources,
};
use alexandria::gpu::{VulkanAttachmentLoadOp, VulkanFormat};

impl FrameGraphBuilderResources {
    /// Create a new transient resource
    pub fn create_resource(
        &mut self,
        name: &'static str,
        format: VulkanFormat,
        kind: FrameGraphResourceKind,
        initial_load_op: VulkanAttachmentLoadOp,
    ) -> FrameGraphResourceId {
        match kind {
            FrameGraphResourceKind::Native => {
                self.native_scale_resources
                    .push(FrameGraphTransientResourceInfo::new(
                        name,
                        format,
                        initial_load_op,
                    ));
                FrameGraphResourceId::new_transient_native_scale(
                    self.native_scale_resources.len() - 1,
                )
            }
            FrameGraphResourceKind::Render => {
                self.render_scale_resources
                    .push(FrameGraphTransientResourceInfo::new(
                        name,
                        format,
                        initial_load_op,
                    ));
                FrameGraphResourceId::new_transient_render_scale(
                    self.render_scale_resources.len() - 1,
                )
            }
        }
    }
}
