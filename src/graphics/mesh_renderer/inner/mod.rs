use crate::graphics::{Material, Mesh, Transform};
use win32::{ComPtr, d3d11::ID3D11Buffer};

mod active;
mod draw;
mod get;
mod instances;
mod new;

/// The actual definition of a mesh renderer
pub struct MeshRendererInner {
    /// Is this camera active?
    active: bool,

    /// The material this renderer uses
    material: Material,

    /// The mesh this material uses
    mesh: Mesh,

    /// The current registered instances
    instances: Vec<Transform>,

    /// Did the instance order change this frame?
    dirty: bool,

    /// The maximum number of instances allowed
    max_instances: usize,

    /// The buffer for the instance matrices
    instance_buffer: ComPtr<ID3D11Buffer>,
}
