use crate::{
    Result,
    graphics::{GraphicsContext, PostProcessingShader, ShaderSource},
};

impl GraphicsContext {
    /// Create a new [`PostProcessingShader`]
    pub fn create_post_processing_shader(
        &mut self,
        shader: &ShaderSource,
    ) -> Result<PostProcessingShader> {
        PostProcessingShader::new(shader, &self.device)
    }
}
