use crate::graphics::{MeshRendererInner, Shader};
use cb_content::MaterialCbContent;
use std::{cell::RefCell, rc::Rc};
use win32::{ComPtr, d3d11::ID3D11Buffer};

mod cb_content;

mod get;
mod mesh_renderers;
mod new;
mod render;
mod set;
mod shader;

/// The actual definition of a material
pub struct MaterialInner {
    /// The shader used by this material
    shader: Shader,

    /// Has the elements of this material changed?
    dirty: bool,

    /// The content of the constant buffer
    buffer_content: MaterialCbContent,

    /// The buffer for material properties on the GPU
    buffer: ComPtr<ID3D11Buffer>,

    /// The mesh renderers that have been registered
    mesh_renderers: Vec<Rc<RefCell<MeshRendererInner>>>,
}
