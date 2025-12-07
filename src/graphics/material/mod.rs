use crate::{
    graphics::{Shader, util::ConstantBuffer},
    util::Handle,
};
use cb_content::MaterialCbContent;
use std::num::NonZeroU32;

mod cb_content;

mod bind;
mod get;
mod new;
mod set;

/// A handle to a [`Material`]
pub type MaterialHandle = Handle<Material>;

/// A item which controls the way meshes are rendered
pub struct Material {
    /// The ID assigned by the graphics context which uniquely identifies this material
    id: NonZeroU32,

    /// The shader used by this material
    shader: Shader,

    /// The constant buffer giving shaders access to this material
    buffer: ConstantBuffer<MaterialCbContent>,
}
