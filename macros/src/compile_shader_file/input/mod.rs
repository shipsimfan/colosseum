use proc_macro_util::tokens::Literal;

mod get;
mod parse;

/// The input to the [`crate::compile_shader_file!`] macro
pub struct CompileShaderInput {
    /// The content containing the shader code
    file_name: Literal,

    /// The type of shader being compiled
    r#type: Literal,

    /// The name of the main function
    main: Literal,
}
