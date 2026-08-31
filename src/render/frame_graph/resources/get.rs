use crate::render::frame_graph::{FrameGraphResource, FrameGraphResourceId, FrameGraphResources};
use alexandria::gpu::{VulkanAttachmentLoadOp, VulkanDescriptorSet};

impl<'a> FrameGraphResources<'a> {
    /// Get a reference to a resource by its ID
    pub fn get<'b>(&'b self, id: FrameGraphResourceId) -> FrameGraphResource<'a, 'b> {
        if id.is_external() {
            FrameGraphResource::External(&self.external[id.index()])
        } else if id.is_transient_render_scale() {
            FrameGraphResource::Transient(&self.transient.render_scale[id.index()])
        } else if id.is_transient_native_scale() {
            FrameGraphResource::Transient(&self.transient.native_scale[id.index()])
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
            &self.transient.render_scale[id.index()]
        } else if id.is_transient_native_scale() {
            &self.transient.native_scale[id.index()]
        } else {
            todo!("transient static resources are not yet implemented")
        };

        let load_op = transient.load_op();
        (FrameGraphResource::Transient(transient), load_op)
    }

    /// Get a reference to a descriptor set by its index in the descriptor set array
    pub fn descriptor_set(&self, index: usize) -> &VulkanDescriptorSet {
        &self.transient.descriptor_sets[index]
    }
}
