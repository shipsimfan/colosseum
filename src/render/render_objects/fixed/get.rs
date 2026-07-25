use crate::render::{FixedRenderObjects, Pipeline, Shader};
use std::sync::Arc;

impl FixedRenderObjects {
    /// Get the fullscreen quad shader
    pub fn fullscreen_quad(&self) -> &Arc<Shader> {
        &self.fullscreen_quad
    }

    /// Get the list of pipelines created for frame graph nodes that don't use materials
    pub fn pipelines(&self) -> &[Pipeline] {
        &self.pipelines
    }
}
