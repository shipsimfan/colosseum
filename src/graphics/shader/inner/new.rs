use crate::{
    Result,
    graphics::{
        ShaderInner, ShaderSource, Vertex,
        util::{PixelShader, VertexShader},
    },
};
use std::num::NonZeroU32;
use win32::d3d11::{D3D11_INPUT_ELEMENT_DESC, ID3D11Device};

impl ShaderInner {
    /// Create a new unlit [`ShaderInner`]
    pub(in crate::graphics::shader) fn new_unlit(
        id: NonZeroU32,
        compiled_shader: &ShaderSource,
        device: &ID3D11Device,
    ) -> Result<Self> {
        Self::new(id, compiled_shader, device, Vertex::UNLIT_INPUT_LAYOUT)
    }

    /// Create a new lit [`ShaderInner`]
    pub(in crate::graphics::shader) fn new_lit(
        id: NonZeroU32,
        compiled_shader: &ShaderSource,
        device: &ID3D11Device,
    ) -> Result<Self> {
        Self::new(id, compiled_shader, device, Vertex::LIT_INPUT_LAYOUT)
    }

    /// Create a new [`ShaderInner`]
    fn new(
        id: NonZeroU32,
        compiled_shader: &ShaderSource,
        device: &ID3D11Device,
        input_layout: &[D3D11_INPUT_ELEMENT_DESC],
    ) -> Result<Self> {
        let vertex_shader =
            VertexShader::new(device, compiled_shader.vertex_content(), input_layout)?;
        let pixel_shader = PixelShader::new(device, compiled_shader.pixel_content())?;

        Ok(ShaderInner {
            id,
            vertex_shader,
            pixel_shader,
        })
    }
}
