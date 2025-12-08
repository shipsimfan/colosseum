use crate::compile_shader_file::CompileShaderFile;
use proc_macro_util::{Generator, ToTokens, to_tokens};

impl ToTokens for CompileShaderFile {
    fn to_tokens(self, generator: &mut Generator) {
        let CompileShaderFile { content, r#type } = self;

        to_tokens! { generator
            colosseum::graphics::ShaderSource::new(
                ::std::borrow::Cow::Borrowed(#content),
                #r#type,
            )
        }
    }
}
