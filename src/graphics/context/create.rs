use crate::{
    Result,
    graphics::{GraphicsContext, Shader, ShaderSource},
};

impl GraphicsContext {
    /// Create a new unlit [`Shader`]
    pub fn create_unlit_shader(&mut self, source: &ShaderSource) -> Result<Shader> {
        Shader::new_unlit(source, &self.device)
    }

    /// Create a new lit [`Shader`]
    pub fn create_lit_shader(&mut self, source: &ShaderSource) -> Result<Shader> {
        Shader::new_lit(source, &self.device)
    }
}
