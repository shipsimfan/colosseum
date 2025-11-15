use crate::{
    self as colosseum, Result,
    graphics::{Shader, ShaderSource},
};
use colosseum_macros::compile_shader_file;
use std::num::NonZeroU32;
use win32::d3d11::ID3D11Device;

const DEFAULT_SHADER: ShaderSource =
    compile_shader_file!("default_unlit.hlsl", "vertex_main", "pixel_main");

impl Shader {
    /// Create the default shader
    pub(in crate::graphics) fn create_default(
        id: NonZeroU32,
        device: &ID3D11Device,
    ) -> Result<Shader> {
        Shader::new_unlit(id, &DEFAULT_SHADER, device)
    }
}
