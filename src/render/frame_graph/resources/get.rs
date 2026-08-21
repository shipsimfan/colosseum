use crate::render::frame_graph::{FrameGraphResource, FrameGraphResourceId, FrameGraphResources};
use alexandria::gpu::VulkanAttachmentLoadOp;

impl<'a> FrameGraphResources<'a> {
    /// Get a reference to a resource by its ID
    pub fn get<'b>(&'b self, id: FrameGraphResourceId) -> FrameGraphResource<'a, 'b> {
        if id.is_external() {
            FrameGraphResource::External(&self.external[id.index()])
        } else if id.is_transient_render_scale() {
            FrameGraphResource::Transient(&self.transient_render_scale[id.index()])
        } else if id.is_transient_native_scale() {
            todo!("transient native scale resources are not yet implemented")
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
            (FrameGraphResource::External(external), load_op)
        } else if id.is_transient_render_scale() {
            let transient = &self.transient_render_scale[id.index()];
            let load_op = transient.load_op();
            (FrameGraphResource::Transient(transient), load_op)
        } else if id.is_transient_native_scale() {
            todo!("transient native scale resources are not yet implemented")
        } else {
            todo!("transient static resources are not yet implemented")
        }
    }
}
