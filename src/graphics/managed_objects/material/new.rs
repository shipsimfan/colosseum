use crate::{
    Result,
    graphics::{
        Material,
        managed_objects::material::{MaterialCbContent, MaterialShader},
        util::ConstantBuffer,
    },
    math::Color3f,
};
use std::{num::NonZeroU32, rc::Rc};
use win32::d3d11::ID3D11Device;

impl Material {
    /// Create a new [`Material`]
    pub(in crate::graphics::managed_objects::material) fn new(
        id: NonZeroU32,
        shader: Rc<MaterialShader>,
        color: Color3f,
        specular_strength: f32,
        device: &ID3D11Device,
    ) -> Result<Self> {
        let buffer_content = MaterialCbContent::new(color, specular_strength);
        let buffer = ConstantBuffer::new(buffer_content, 1, device)?;

        Ok(Material { id, shader, buffer })
    }
}
