use std::rc::Rc;

mod inner;

mod deref;
mod new;

pub use inner::MeshInner;

/// A 3d model
#[derive(Clone)]
pub struct Mesh {
    /// The actual mesh
    inner: Rc<MeshInner>,
}
