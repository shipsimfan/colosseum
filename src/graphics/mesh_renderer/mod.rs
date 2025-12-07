use crate::{
    graphics::{MaterialHandle, Mesh, util::InstanceBuffer},
    math::Matrix4x4f,
    util::Handle,
};

mod active;
mod draw;
mod get;
mod instances;
mod new;
mod set;

/// A handle to a [`MeshRenderer`]
pub type MeshRendererHandle = Handle<MeshRenderer>;

/// A item which renders multiple copies of the same mesh
pub struct MeshRenderer {
    /// Is this camera active?
    active: bool,

    /// The material this renderer uses
    material: MaterialHandle,

    /// The mesh this material uses
    mesh: Mesh,

    /// The number of instances that are currently active
    active_instances: usize,

    /// The buffer for the instance matrices
    instance_buffer: InstanceBuffer<Matrix4x4f>,
}
