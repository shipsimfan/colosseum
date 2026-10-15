use crate::{
    Result,
    render::{FullscreenQuadShader, GraphicsDevice, Shader, ShaderKind},
};
use alexandria::gpu::compile_shader;

compile_shader! {
    /// The vertex shader code for the fullscreen quad shader
    const FULLSCREEN_QUAD_SHADER = "fullscreen-quad.slang",
    vert_main
}

impl FullscreenQuadShader {
    /// Creates a new instance of the fullscreen quad shader
    pub fn new(device: &GraphicsDevice) -> Result<FullscreenQuadShader> {
        let shader = Shader::new(&FULLSCREEN_QUAD_SHADER, ShaderKind::Other, device)?;
        Ok(FullscreenQuadShader { shader })
    }
}
