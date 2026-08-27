use crate::render::frame_graph::{FrameGraphResource, FrameGraphResourceId, FrameGraphResources};
use alexandria::gpu::VulkanAttachmentLoadOp;

impl<'a> FrameGraphResources<'a> {
    /// Get a reference to a resource by its ID
    pub fn get<'b>(&'b self, id: FrameGraphResourceId) -> FrameGraphResource<'a, 'b> {
        if id.is_external() {
            FrameGraphResource::External(&self.external[id.index()])
        } else if id.is_transient_render_scale() {
            FrameGraphResource::Transient(&self.render_scale_transients[id.index()])
        } else if id.is_transient_native_scale() {
            FrameGraphResource::Transient(&self.native_scale_transients[id.index()])
        } else {
            todo!("transient static resources are not yet implemented")
        }
    }

    /// Get a reference to a resource by its ID, along with the load operation to use
    pub fn get_with_op<'b>(
        &'b self,
        id: FrameGraphResourceId,
    ) -> (FrameGraphResource<'a, 'b>, VulkanAttachmentLoadOp) {
        if id.is_external() {
            let external = &self.external[id.index()];
            let load_op = external.load_op();
            return (FrameGraphResource::External(external), load_op);
        }

        let transient = if id.is_transient_render_scale() {
            &self.render_scale_transients[id.index()]
        } else if id.is_transient_native_scale() {
            &self.native_scale_transients[id.index()]
        } else {
            todo!("transient static resources are not yet implemented")
        };

        let load_op = transient.load_op();
        (FrameGraphResource::Transient(transient), load_op)
    }
}
