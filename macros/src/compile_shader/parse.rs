use crate::{
    compile_shader::{CompileShader, input::CompileShaderInput},
    d3d_compile::d3d_compile,
};
use proc_macro_util::{Parse, Parser, Result, tokens::Literal};

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

impl<'a> Parse<'a> for CompileShader {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let input = CompileShaderInput::parse(parser)?;
        let content = strip_string(input.content())?;

        let vertex_content = d3d_compile(&content, &strip_string(input.vertex_main())?, c"vs_5_0")
            .map_err(|error| {
                input
                    .content()
                    .span()
                    .error(format!("unable to compile program - {}", error))
            })?;
        let pixel_content = d3d_compile(&content, &strip_string(input.pixel_main())?, c"ps_5_0")
            .map_err(|error| {
                input
                    .content()
                    .span()
                    .error(format!("unable to compile program - {}", error))
            })?;

        Ok(CompileShader {
            vertex_content: Literal::new(vertex_content.as_slice()),
            pixel_content: Literal::new(pixel_content.as_slice()),
        })
    }
}
