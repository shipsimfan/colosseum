use crate::{
    self as colosseum, Result,
    graphics::{Shader, ShaderSource},
};
use colosseum_macros::compile_shader_file;
use std::num::NonZeroU32;
use win32::d3d11::ID3D11Device;

const DEFAULT_LIT_SHADER: ShaderSource =
    compile_shader_file!("default_lit.hlsl", "vertex_main", "pixel_main");

impl Shader {
    /// Create the default lit shader
    pub(in crate::graphics) fn create_default_lit(
        id: NonZeroU32,
        device: &ID3D11Device,
    ) -> Result<Shader> {
        Shader::new_lit(id, &DEFAULT_LIT_SHADER, device)
    }
}
