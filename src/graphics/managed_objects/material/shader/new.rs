use crate::{
    self as colosseum, Result,
    graphics::{
        ShaderSource, Vertex,
        managed_objects::material::MaterialShader,
        util::{PixelShader, VertexShader},
    },
};
use colosseum_macros::compile_shader_file;
use std::rc::Rc;
use win32::d3d11::{D3D11_INPUT_ELEMENT_DESC, ID3D11Device};

const LIT_VERTEX_SHADER: ShaderSource = compile_shader_file!("lit.hlsl", "vs_5_0", "vertex_main");
const LIT_PIXEL_SHADER: ShaderSource = compile_shader_file!("lit.hlsl", "ps_5_0", "pixel_main");
const UNLIT_VERTEX_SHADER: ShaderSource =
    compile_shader_file!("unlit.hlsl", "vs_5_0", "vertex_main");
const UNLIT_PIXEL_SHADER: ShaderSource = compile_shader_file!("unlit.hlsl", "ps_5_0", "pixel_main");

impl MaterialShader {
    /// Create a new lit [`MaterialShader`]
    pub(in crate::graphics::managed_objects::material) fn new_lit(
        device: &ID3D11Device,
    ) -> Result<Rc<Self>> {
        Self::new(
            &LIT_VERTEX_SHADER,
            &LIT_PIXEL_SHADER,
            device,
            Vertex::LIT_INPUT_LAYOUT,
        )
    }

    /// Create a new unlit [`MaterialShader`]
    pub(in crate::graphics::managed_objects::material) fn new_unlit(
        device: &ID3D11Device,
    ) -> Result<Rc<Self>> {
        Self::new(
            &UNLIT_VERTEX_SHADER,
            &UNLIT_PIXEL_SHADER,
            device,
            Vertex::UNLIT_INPUT_LAYOUT,
        )
    }

    /// Create a new [`MaterialShader`]
    fn new(
        vertex_shader: &ShaderSource,
        pixel_shader: &ShaderSource,
        device: &ID3D11Device,
        input_layout: &[D3D11_INPUT_ELEMENT_DESC],
    ) -> Result<Rc<Self>> {
        assert_eq!(
            vertex_shader.r#type(),
            "vs_5_0",
            "invalid vertex shader type"
        );
        assert_eq!(pixel_shader.r#type(), "ps_5_0", "invalid pixel shader type");

        let vertex_shader = VertexShader::new(device, vertex_shader.content(), input_layout)?;
        let pixel_shader = PixelShader::new(device, pixel_shader.content())?;

        Ok(Rc::new(MaterialShader {
            vertex_shader,
            pixel_shader,
        }))
    }
}
