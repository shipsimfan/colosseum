use crate::render::{FullscreenQuadShader, Shader};
use std::sync::Arc;

impl FullscreenQuadShader {
    /// Get a reference to the underlying shader
    pub fn get(&self) -> &Arc<Shader> {
        &self.shader
    }
}
