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

        let content = d3d_compile(
            &content,
            &strip_string(input.main())?,
            &strip_string(input.r#type())?,
        )
        .map_err(|error| {
            input
                .content()
                .span()
                .error(format!("unable to compile program - {}", error))
        })?;

        Ok(CompileShader {
            content: Literal::new(content.as_slice()),
            r#type: input.r#type().clone(),
        })
    }
}
