use crate::graphics::{MaterialInner, MeshRendererInner};
use std::{cell::RefCell, rc::Rc};

impl MaterialInner {
    /// Add a new [`MeshRendererInner`] to this material's list
    pub(in crate::graphics) fn push_mesh_renderer(
        &mut self,
        mesh_renderer: Rc<RefCell<MeshRendererInner>>,
    ) {
        self.mesh_renderers.push(mesh_renderer);
    }

    pub(in crate::graphics) fn remove_mesh_renderer(
        &mut self,
        mesh_renderer: &Rc<RefCell<MeshRendererInner>>,
    ) {
        let mut found = None;
        for (i, renderer) in self.mesh_renderers.iter().enumerate() {
            if Rc::ptr_eq(mesh_renderer, renderer) {
                found = Some(i);
                break;
            }
        }

        if let Some(i) = found {
            self.mesh_renderers.swap_remove(i);
        }
    }
}
