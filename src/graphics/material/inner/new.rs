use crate::{
    Result,
    graphics::{MaterialInner, Shader, material::inner::MaterialCbContent, util::ConstantBuffer},
    math::Color3f,
};
use win32::d3d11::ID3D11Device;

impl MaterialInner {
    /// Create a new [`MaterialInner`]
    pub(in crate::graphics::material) fn new(
        shader: Shader,
        color: Color3f,
        specular_strength: f32,
        device: &ID3D11Device,
    ) -> Result<Self> {
        let buffer_content = MaterialCbContent::new(color, specular_strength);
        let buffer = ConstantBuffer::new(buffer_content, 1, device)?;

        Ok(MaterialInner {
            shader,
            buffer,
            mesh_renderers: Vec::new(),
        })
    }
}
