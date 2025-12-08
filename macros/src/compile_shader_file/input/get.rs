use crate::compile_shader_file::input::CompileShaderInput;
use proc_macro_util::tokens::Literal;

impl CompileShaderInput {
    /// Get the name of the file to read
    pub fn file_name(&self) -> &Literal {
        &self.file_name
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
