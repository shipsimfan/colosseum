use crate::{
    compile_shader_file::{CompileShaderFile, input::CompileShaderInput},
    d3d_compile::d3d_compile,
};
use proc_macro_util::{Parse, Parser, Result, tokens::Literal};
use std::path::Path;

fn strip_string(literal: &Literal) -> Result<String> {
    let content = literal.to_string();
    match content.strip_prefix("r\"") {
        Some(content) => Ok(content[..content.len() - 1].to_string()),
        None => match content.strip_prefix("\"") {
            Some(content) => Ok(content[..content.len() - 1].to_string()),
            None => Err(literal.span().error("expected a string literals")),
        },
    }
}

impl<'a> Parse<'a> for CompileShaderFile {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let input = CompileShaderInput::parse(parser)?;

        let path = Path::new(&input.file_name().span().file())
            .parent()
            .unwrap_or(Path::new(""))
            .join(strip_string(input.file_name())?);
        let content = std::fs::read_to_string(&path).map_err(|error| {
            input.file_name().span().error(format!(
                "unable to read \"{}\" - {}",
                path.display(),
                error
            ))
        })?;

        let vertex_content = d3d_compile(&content, &strip_string(input.vertex_main())?, c"vs_5_0")
            .map_err(|error| {
                input
                    .file_name()
                    .span()
                    .error(format!("unable to compile program - {}", error))
            })?;
        let pixel_content = d3d_compile(&content, &strip_string(input.pixel_main())?, c"ps_5_0")
            .map_err(|error| {
                input
                    .file_name()
                    .span()
                    .error(format!("unable to compile program - {}", error))
            })?;

        Ok(CompileShaderFile {
            vertex_content: Literal::new(vertex_content.as_slice()),
            pixel_content: Literal::new(pixel_content.as_slice()),
        })
    }
}
