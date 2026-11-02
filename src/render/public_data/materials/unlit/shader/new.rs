use crate::{
    Result,
    render::{GraphicsDevice, Shader, ShaderCode, UnlitShader},
};
use std::sync::Arc;

impl UnlitShader {
    pub(in crate::render::public_data::materials) fn new<const N: usize>(
        code: &ShaderCode<N>,
        device: &GraphicsDevice,
    ) -> Result<Arc<UnlitShader>> {
        let shader = Shader::new(code, device)?;

        todo!("create unlit pipeline")
    }
}
