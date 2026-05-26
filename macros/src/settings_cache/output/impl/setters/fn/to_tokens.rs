use crate::settings_cache::SettingsCacheOutputSetterFn;
use proc_macro_util::{Generator, ToTokens, to_tokens};

impl<'a> ToTokens for SettingsCacheOutputSetterFn<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        let SettingsCacheOutputSetterFn {
            fn_name,
            field_name,
            r#type,
        } = self;

        let field_name2 = field_name.clone();

        to_tokens! { generator
            #[doc = ::std::concat!("Set the `", ::std::stringify!(#field_name2), "` settings group")]
            pub fn #fn_name(&mut self, value: #r#type) {
                self.#field_name = value;
            }
        }
    }
}
