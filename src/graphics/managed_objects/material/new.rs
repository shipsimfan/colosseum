use crate::{
    Result,
    graphics::{
        Material, Shader, managed_objects::material::MaterialCbContent, util::ConstantBuffer,
    },
    math::Color3f,
};
use std::num::NonZeroU32;
use win32::d3d11::ID3D11Device;

impl Material {
    /// Create a new [`Material`]
    pub(in crate::graphics) fn new(
        id: NonZeroU32,
        shader: Shader,
        color: Color3f,
        specular_strength: f32,
        device: &ID3D11Device,
    ) -> Result<Self> {
        let buffer_content = MaterialCbContent::new(color, specular_strength);
        let buffer = ConstantBuffer::new(buffer_content, 1, device)?;

        Ok(Material { id, shader, buffer })
    }
}
