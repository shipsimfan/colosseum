use crate::{
    self as colosseum, Result,
    graphics::{
        Vertex,
        managed_objects::material::{MaterialShader, shader::ShaderSource},
        util::{PixelShader, VertexShader},
    },
};
use colosseum_macros::compile_shader_file;
use std::rc::Rc;
use win32::d3d11::{D3D11_INPUT_ELEMENT_DESC, ID3D11Device};

const LIT_SHADER: ShaderSource = compile_shader_file!("lit.hlsl", "vertex_main", "pixel_main");
const UNLIT_SHADER: ShaderSource = compile_shader_file!("unlit.hlsl", "vertex_main", "pixel_main");

impl MaterialShader {
    /// Create a new lit [`MaterialShader`]
    pub(in crate::graphics::managed_objects::material) fn new_lit(
        device: &ID3D11Device,
    ) -> Result<Rc<Self>> {
        Self::new(&LIT_SHADER, device, Vertex::LIT_INPUT_LAYOUT)
    }

    /// Create a new unlit [`MaterialShader`]
    pub(in crate::graphics::managed_objects::material) fn new_unlit(
        device: &ID3D11Device,
    ) -> Result<Rc<Self>> {
        Self::new(&UNLIT_SHADER, device, Vertex::UNLIT_INPUT_LAYOUT)
    }

    /// Create a new [`MaterialShader`]
    fn new(
        compiled_shader: &ShaderSource,
        device: &ID3D11Device,
        input_layout: &[D3D11_INPUT_ELEMENT_DESC],
    ) -> Result<Rc<Self>> {
        let vertex_shader =
            VertexShader::new(device, compiled_shader.vertex_content(), input_layout)?;
        let pixel_shader = PixelShader::new(device, compiled_shader.pixel_content())?;

        Ok(Rc::new(MaterialShader {
            vertex_shader,
            pixel_shader,
        }))
    }
}
