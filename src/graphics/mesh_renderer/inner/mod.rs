use crate::graphics::{Material, Mesh};

mod active;
mod draw;
mod get;
mod new;

/// The actual definition of a mesh renderer
pub struct MeshRendererInner {
    /// Is this camera active?
    active: bool,

    /// The material this renderer uses
    material: Material,

    /// The mesh this material uses
    mesh: Mesh,
}
