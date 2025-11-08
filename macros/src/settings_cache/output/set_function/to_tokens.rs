use crate::settings_cache::output::SettingsCacheSetFunction;
use proc_macro_util::{Generator, ToTokens, to_tokens};

impl<'a> ToTokens for SettingsCacheSetFunction<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        let SettingsCacheSetFunction {
            function_name,
            field_name,
            field_type,
        } = self;

        to_tokens! { generator
            #[allow(missing_docs)]
            pub fn #function_name(&mut self, #field_name: &#field_type) {
                unimplemented!();
            }
        }
    }
}
