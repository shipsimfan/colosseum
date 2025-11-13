use crate::{
    graphics::{MeshRendererInner, Shader},
    math::Color3f,
};
use std::{cell::RefCell, rc::Rc};
use win32::{ComPtr, d3d11::ID3D11Buffer};

mod color;
mod mesh_renderers;
mod new;
mod render;
mod shader;

/// The actual definition of a material
pub struct MaterialInner {
    /// The shader used by this material
    shader: Shader,

    /// The color assigned to all objects using this material
    color: Color3f,

    /// Has the elements of this material changed?
    dirty: bool,

    /// The buffer for material properties on the GPU
    buffer: ComPtr<ID3D11Buffer>,

    /// The mesh renderers that have been registered
    mesh_renderers: Vec<Rc<RefCell<MeshRendererInner>>>,
}
