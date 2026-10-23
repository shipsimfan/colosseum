use crate::{
    Result,
    render::{AssetLoader, Shader, ShaderCode, ShaderKind},
};
use std::sync::Arc;

impl AssetLoader {
    /// Create a new [`Shader`]
    pub fn create_shader<const N: usize>(
        &self,
        code: &ShaderCode<N>,
        kind: ShaderKind,
    ) -> Result<Arc<Shader>> {
        Shader::new(code, kind, &self.device)
    }
}
