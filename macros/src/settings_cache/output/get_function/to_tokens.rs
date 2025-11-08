use crate::settings_cache::output::SettingsCacheGetFunction;
use proc_macro_util::{Generator, ToTokens, to_tokens};

impl<'a> ToTokens for SettingsCacheGetFunction<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        let SettingsCacheGetFunction {
            field_name,
            field_type,
        } = self;

        let field_name2 = field_name.clone();

        to_tokens! { generator
            #[allow(missing_docs)]
            pub fn #field_name(&self) -> &#field_type {
                &self.#field_name2
            }
        }
    }
}
