use crate::compile_shader::input::CompileShaderInput;
use proc_macro_util::tokens::Literal;

impl CompileShaderInput {
    /// Get the provided content
    pub fn content(&self) -> &Literal {
        &self.content
    }

    /// The type of the shader being compiled
    pub fn r#type(&self) -> &Literal {
        &self.r#type
    }

    /// Get the provided name of the main function
    pub fn main(&self) -> &Literal {
        &self.main
    }
}
