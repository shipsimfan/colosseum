use proc_macro_util::tokens::Literal;

mod input;

mod parse;
mod to_tokens;

pub struct CompileShaderFile {
    /// The compiled content of the vertex shader
    vertex_content: Literal,

    /// The compiled content of the pixel shader
    pixel_content: Literal,
}
