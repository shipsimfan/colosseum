use std::borrow::Cow;

mod get;
mod new;

/// Compiled shader source code
pub struct ShaderSource<'a> {
    /// The raw byte content of the shader
    content: Cow<'a, [u8]>,

    /// The type of this shader
    r#type: &'static str,
}
