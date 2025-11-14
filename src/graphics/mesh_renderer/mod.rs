use std::{cell::RefCell, rc::Rc};

mod inner;

mod borrow;
mod drop;
mod new;

pub use inner::MeshRendererInner;

/// A item which renders multiple copies of the same mesh
#[derive(Clone)]
pub struct MeshRenderer {
    /// The reference to the mesh renderer itself
    mesh_renderer: Rc<RefCell<MeshRendererInner>>,
}
