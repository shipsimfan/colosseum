use crate::graphics::{MeshRendererHandle, MeshRenderers};

impl MeshRenderers {
    /// Remove the [`MeshRenderer`](crate::graphics::MeshRenderer) identified by `handle`
    pub fn remove(&mut self, handle: MeshRendererHandle) {
        self.arena.remove(handle);
    }
}
