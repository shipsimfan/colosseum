use crate::graphics::{Material, Mesh, MeshRenderer, MeshRendererInner};
use std::{cell::RefCell, rc::Rc};

impl MeshRenderer {
    /// Create a new [`MeshRenderer`]
    pub fn new(material: Material, mesh: Mesh) -> Self {
        let mesh_renderer = Rc::new(RefCell::new(MeshRendererInner::new(material.clone(), mesh)));

        material
            .borrow_mut()
            .push_mesh_renderer(mesh_renderer.clone());

        MeshRenderer { mesh_renderer }
    }
}
