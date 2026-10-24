use crate::{
    Result,
    render::{Assets, Shader, ShaderCode, ShaderKind},
};
use std::sync::Arc;

impl Assets {
    /// Create a new [`Shader`]
    pub fn create_shader<const N: usize>(
        &self,
        code: &ShaderCode<N>,
        kind: ShaderKind,
    ) -> Result<Arc<Shader>> {
        Shader::new(code, kind, &self.device)
    }
}
