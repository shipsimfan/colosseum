use crate::{
    Result,
    graphics::{Material, Materials, Shader},
    math::Color3f,
    util::Arena,
};
use std::num::NonZeroU32;
use win32::{ComPtr, d3d11::ID3D11Device};

const DEFAULT_SPECULAR_STRENGTH: f32 = 0.5;

impl Materials {
    /// Create a new set of [`Materials`] with a default lit and unlit material
    pub(in crate::graphics::managed_objects) fn new(
        default_lit_shader: Shader,
        default_unlit_shader: Shader,
        device: ComPtr<ID3D11Device>,
    ) -> Result<Self> {
        let mut arena = Arena::new();

        let default_lit = arena.insert(Material::new(
            NonZeroU32::new(1).unwrap(),
            default_lit_shader.clone(),
            Color3f::WHITE,
            DEFAULT_SPECULAR_STRENGTH,
            &device,
        )?);
        let default_unlit = arena.insert(Material::new(
            NonZeroU32::new(2).unwrap(),
            default_unlit_shader.clone(),
            Color3f::WHITE,
            DEFAULT_SPECULAR_STRENGTH,
            &device,
        )?);

        Ok(Materials {
            arena,
            next_material_id: NonZeroU32::new(3).unwrap(),
            device,
            default_lit,
            default_unlit,
        })
    }
}
