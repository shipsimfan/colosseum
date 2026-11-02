use crate::render::{FullscreenQuadShader, Shader};

impl FullscreenQuadShader {
    /// Get a reference to the underlying shader
    pub fn get(&self) -> &Shader {
        &self.shader
    }
}
