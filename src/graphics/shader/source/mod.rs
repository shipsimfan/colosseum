use std::borrow::Cow;

mod get;
mod new;

/// Compiled shader source code
pub struct ShaderSource<'a> {
    /// The raw byte content of the vertex shader
    vertex_content: Cow<'a, [u8]>,

    /// The raw byte content of the pixel shader
    pixel_content: Cow<'a, [u8]>,
}
