use crate::{
    TransformHandle,
    graphics::{MaterialHandle, Mesh, util::InstanceBuffer},
    math::Matrix4x4f,
    util::Handle,
};

mod mesh_renderers;

mod active;
mod draw;
mod get;
mod instances;
mod new;
mod set;

pub use mesh_renderers::MeshRenderers;

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

    /// The set of transforms and the last epoch they were updated at
    instances: Vec<(TransformHandle, u32)>,

    /// The buffer for the instance matrices
    instance_buffer: InstanceBuffer<Matrix4x4f>,
}
