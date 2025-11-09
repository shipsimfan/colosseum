use crate::compile_shader_file::input::CompileShaderInput;
use proc_macro_util::tokens::Literal;

impl CompileShaderInput {
    /// Get the name of the file to read
    pub fn file_name(&self) -> &Literal {
        &self.file_name
    }

    /// Get the provided name of the main vertex function
    pub fn vertex_main(&self) -> &Literal {
        &self.vertex_main
    }

    /// Get the provided name of the main pixel function
    pub fn pixel_main(&self) -> &Literal {
        &self.pixel_main
    }
}
