use proc_macro_util::tokens::Literal;

mod input;

mod parse;
mod to_tokens;

pub struct CompileShader {
    /// The compiled content of the shader
    content: Literal,

    /// The type of the compiled shader
    r#type: Literal,
}
