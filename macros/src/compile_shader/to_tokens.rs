use crate::compile_shader::CompileShader;
use proc_macro_util::{Generator, ToTokens, to_tokens};

impl ToTokens for CompileShader {
    fn to_tokens(self, generator: &mut Generator) {
        let CompileShader { content, r#type } = self;

        to_tokens! { generator
            colosseum::graphics::ShaderSource::new(
                ::std::borrow::Cow::Borrowed(#content),
                #r#type,
            )
        }
    }
}
