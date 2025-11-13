use std::{cell::RefCell, rc::Rc};

mod inner;

mod drop;
mod new;

pub use inner::MeshRendererInner;

/// A item which renders meshes
#[derive(Clone)]
pub struct MeshRenderer {
    /// The reference to the mesh renderer itself
    mesh_renderer: Rc<RefCell<MeshRendererInner>>,
}
