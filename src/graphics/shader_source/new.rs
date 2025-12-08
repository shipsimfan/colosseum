use crate::graphics::ShaderSource;
use std::borrow::Cow;

impl<'a> ShaderSource<'a> {
    /// Create a new [`ShaderSource`]
    pub const fn new(content: Cow<'a, [u8]>, r#type: &'static str) -> Self {
        ShaderSource { content, r#type }
    }
}
