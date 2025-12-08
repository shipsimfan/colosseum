use crate::compile_shader::input::CompileShaderInput;
use proc_macro_util::{Parse, Parser, Result, Token};

impl<'a> Parse<'a> for CompileShaderInput {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let content = parser.parse()?;
        parser.parse::<Token![,]>()?;
        let r#type = parser.parse()?;
        parser.parse::<Token![,]>()?;
        let main = parser.parse()?;

        Ok(CompileShaderInput {
            content,
            r#type,
            main,
        })
    }
}
