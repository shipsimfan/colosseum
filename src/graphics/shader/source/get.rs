use crate::graphics::ShaderSource;

impl<'a> ShaderSource<'a> {
    /// Gets the content of the vertex shader
    pub fn vertex_content(&self) -> &[u8] {
        &self.vertex_content
    }

    /// Gets the content of the pixel shader
    pub fn pixel_content(&self) -> &[u8] {
        &self.pixel_content
    }
}
