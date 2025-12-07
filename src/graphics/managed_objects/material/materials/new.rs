use crate::{
    Result,
    graphics::{Material, Materials, managed_objects::material::MaterialShader},
    math::Color3f,
    util::Arena,
};
use std::num::NonZeroU32;
use win32::{ComPtr, d3d11::ID3D11Device};

const DEFAULT_SPECULAR_STRENGTH: f32 = 0.5;

impl Materials {
    /// Create a new set of [`Materials`] with a default lit and unlit material
    pub(in crate::graphics::managed_objects) fn new(device: ComPtr<ID3D11Device>) -> Result<Self> {
        let mut arena = Arena::new();

        let lit_shader = MaterialShader::new_lit(&device)?;
        let unlit_shader = MaterialShader::new_unlit(&device)?;

        let default_lit = arena.insert(Material::new(
            NonZeroU32::new(1).unwrap(),
            lit_shader.clone(),
            Color3f::WHITE,
            DEFAULT_SPECULAR_STRENGTH,
            &device,
        )?);
        let default_unlit = arena.insert(Material::new(
            NonZeroU32::new(2).unwrap(),
            unlit_shader.clone(),
            Color3f::WHITE,
            DEFAULT_SPECULAR_STRENGTH,
            &device,
        )?);

        Ok(Materials {
            arena,
            next_material_id: NonZeroU32::new(3).unwrap(),
            device,
            lit_shader,
            unlit_shader,
            default_lit,
            default_unlit,
        })
    }
}
