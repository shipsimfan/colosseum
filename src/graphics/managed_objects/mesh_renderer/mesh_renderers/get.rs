use crate::graphics::{MeshRenderer, MeshRendererHandle, MeshRenderers};

impl MeshRenderers {
    /// Get the [`MeshRenderer`] at `handle`
    pub fn get(&self, handle: MeshRendererHandle) -> Option<&MeshRenderer> {
        self.arena.get(handle)
    }

    /// Get the [`MeshRenderer`] at `handle` mutably
    pub fn get_mut(&mut self, handle: MeshRendererHandle) -> Option<&mut MeshRenderer> {
        self.arena.get_mut(handle)
    }
}
