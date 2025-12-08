//! Procedural macros for colosseum

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]
#![feature(proc_macro_diagnostic)]

mod compile_shader;
mod compile_shader_file;
mod settings_cache;

mod d3d_compile;

proc_macro_util::proc_macro_attribute!(
    /// Converts a struct into a settings cache
    settings_cache -> settings_cache::settings_cache
);

proc_macro_util::proc_macro_function!(
    /// Compiles HLSL into a CompiledShader, producing the struct in place of this macro.
    ///
    /// # Format
    /// ```ignore
    /// compile_shader!(content: literal, r#type: literal, main: literal);
    /// ```
    ///
    /// # Parameters
    ///  * `content` - String literal containing the code content
    ///  * `r#type` - The type of shader being compiled (e.g. "vs_5_0", "ps_5_0", etc.)
    ///  * `main` - The name of the main function
    compile_shader -> compile_shader::CompileShader
);

proc_macro_util::proc_macro_function!(
    /// Compiles an HLSL file into a CompiledShader, producing the struct in place of this macro.
    ///
    /// # Format
    /// ```ignore
    /// compile_shader_file!(file_name: literal, r#type: literal, main: literal);
    /// ```
    ///
    /// # Parameters
    ///  * `file_name` - The name of the file, relative to the defining module file
    ///  * `r#type` - The type of shader being compiled (e.g. "vs_5_0", "ps_5_0", etc.)
    ///  * `main` - The name of the main function
    compile_shader_file -> compile_shader_file::CompileShaderFile
);
