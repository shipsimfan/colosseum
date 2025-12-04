use crate::graphics::{MeshRendererInner, Shader, util::ConstantBuffer};
use cb_content::MaterialCbContent;
use std::{cell::RefCell, rc::Rc};

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

    /// The constant buffer giving shaders access to this material
    buffer: ConstantBuffer<MaterialCbContent>,

    /// The mesh renderers that have been registered
    mesh_renderers: Vec<Rc<RefCell<MeshRendererInner>>>,
}
