use crate::compile_shader::CompileShader;
use proc_macro_util::{Generator, ToTokens, to_tokens};

impl ToTokens for CompileShader {
    fn to_tokens(self, generator: &mut Generator) {
        let CompileShader {
            vertex_content,
            pixel_content,
        } = self;

        to_tokens! { generator
            colosseum::graphics::managed_objects::material::shader::ShaderSource::new(
                ::std::borrow::Cow::Borrowed(#vertex_content),
                ::std::borrow::Cow::Borrowed(#pixel_content),
            )
        }
    }
}
