use crate::{graphics::util::ConstantBuffer, util::Handle};
use cb_content::MaterialCbContent;
use shader::MaterialShader;
use std::{num::NonZeroU32, rc::Rc};

mod cb_content;
mod materials;
mod shader;

mod bind;
mod get;
mod new;
mod set;

pub use materials::Materials;

/// A handle to a [`Material`]
pub type MaterialHandle = Handle<Material>;

/// A item which controls the way meshes are rendered
pub struct Material {
    /// The ID assigned by the graphics context which uniquely identifies this material
    id: NonZeroU32,

    /// The shader used by this material
    shader: Rc<MaterialShader>,

    /// The constant buffer giving shaders access to this material
    buffer: ConstantBuffer<MaterialCbContent>,
}
