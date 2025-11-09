use crate::graphics::ShaderSource;
use std::borrow::Cow;

impl<'a> ShaderSource<'a> {
    /// Create a new [`ShaderSource`]
    pub const fn new(vertex_content: Cow<'a, [u8]>, pixel_content: Cow<'a, [u8]>) -> Self {
        ShaderSource {
            vertex_content,
            pixel_content,
        }
    }
}
