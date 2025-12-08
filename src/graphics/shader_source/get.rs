use crate::graphics::ShaderSource;

impl<'a> ShaderSource<'a> {
    /// Gets the content of the shader
    pub fn content(&self) -> &[u8] {
        &self.content
    }

    /// Get the type of this shader
    pub fn r#type(&self) -> &'static str {
        self.r#type
    }
}
