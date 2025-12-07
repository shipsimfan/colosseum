use crate::{
    graphics::{Material, MaterialHandle, managed_objects::material::MaterialShader},
    util::Arena,
};
use std::{num::NonZeroU32, rc::Rc};
use win32::{ComPtr, d3d11::ID3D11Device};

mod create;
mod get;
mod index;
mod iter;
mod new;
mod remove;

/// A set of [`Material`]s registered with the engine
pub struct Materials {
    /// The set of [`Material`]s
    arena: Arena<Material>,

    /// The ID to assign the next [`Material`]
    next_material_id: NonZeroU32,

    /// The device for creating new [`Material`]s
    device: ComPtr<ID3D11Device>,

    /// The shader to use for lit materials
    lit_shader: Rc<MaterialShader>,

    /// The shader to use for unlit materials
    unlit_shader: Rc<MaterialShader>,

    /// The default lit material
    default_lit: MaterialHandle,

    /// The default unlit material
    default_unlit: MaterialHandle,
}
