use std::rc::Rc;

mod inner;
mod primtives;

mod deref;
mod new;

pub use inner::MeshInner;
pub use primtives::MeshPrimitives;

/// A 3d model
#[derive(Clone)]
pub struct Mesh {
    /// The actual mesh
    inner: Rc<MeshInner>,
}
