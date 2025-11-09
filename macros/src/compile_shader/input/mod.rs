use proc_macro_util::tokens::Literal;

mod get;
mod parse;

/// The input to the [`crate::compile_shader!`] macro
pub struct CompileShaderInput {
    /// The content containing the shader code
    content: Literal,

    /// The name of the main vertex function
    vertex_main: Literal,

    /// The name of the main pixel function
    pixel_main: Literal,
}
