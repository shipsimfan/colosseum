use crate::graphics::{MeshRenderer, MeshRendererInner};
use std::cell::{Ref, RefMut};

impl MeshRenderer {
    /// Get immutable access to the mesh renderer
    pub fn borrow<'a>(&'a self) -> Ref<'a, MeshRendererInner> {
        self.mesh_renderer.borrow()
    }

    /// Get mutable access to the mesh renderer
    pub fn borrow_mut<'a>(&'a self) -> RefMut<'a, MeshRendererInner> {
        self.mesh_renderer.borrow_mut()
    }
}
