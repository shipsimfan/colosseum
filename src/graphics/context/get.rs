use crate::graphics::{GraphicsContext, Shader};

impl GraphicsContext {
    /// Get the default lit shader
    pub fn default_lit_shader(&self) -> Shader {
        self.default_lit_shader.clone()
    }

    /// Get the default unlit shader
    pub fn default_unlit_shader(&self) -> Shader {
        self.default_unlit_shader.clone()
    }
}
