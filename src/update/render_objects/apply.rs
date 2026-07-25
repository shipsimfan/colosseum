use crate::{render::RenderObjectRemoveConfirm, update::UpdateRenderObjects};

impl UpdateRenderObjects {
    /// Apply a removal of a render object to the update render objects
    pub(crate) fn apply_removal(&mut self, removal: RenderObjectRemoveConfirm) {
        match removal {
            RenderObjectRemoveConfirm::Mesh(memory) => self.mesh_allocator.free(memory),
        }
    }
}
