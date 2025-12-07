use crate::{
    self as colosseum, Result,
    graphics::{Shader, ShaderSource},
};
use colosseum_macros::compile_shader_file;
use win32::d3d11::ID3D11Device;

const DEFAULT_UNLIT_SHADER: ShaderSource =
    compile_shader_file!("default_unlit.hlsl", "vertex_main", "pixel_main");

impl Shader {
    /// Create the default unlit shader
    pub(in crate::graphics) fn create_default_unlit(device: &ID3D11Device) -> Result<Shader> {
        Shader::new_unlit(&DEFAULT_UNLIT_SHADER, device)
    }
}
