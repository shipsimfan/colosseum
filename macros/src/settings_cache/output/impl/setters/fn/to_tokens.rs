use crate::settings_cache::SettingsCacheOutputSetterFn;
use proc_macro_util::{Generator, ToTokens, to_tokens};

impl<'a> ToTokens for SettingsCacheOutputSetterFn<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        let SettingsCacheOutputSetterFn {
            fn_name,
            mut_fn_name,
            field_name,
            r#type,
        } = self;

        let field_name2 = field_name.clone();
        let field_name3 = field_name.clone();
        let field_name4 = field_name.clone();
        let field_name5 = field_name.clone();
        let field_name6 = field_name.clone();

        let r#type2 = r#type.clone();

        to_tokens! { generator
            #[doc = ::std::concat!("Set the `", ::std::stringify!(#field_name), "` settings group")]
            pub fn #fn_name(&mut self, value: #r#type) {
                self.#field_name2.0 = value;
                self.#field_name3.1 = true;
            }

            #[doc = ::std::concat!("Get a mutable reference to the `", ::std::stringify!(#field_name4), "` settings group")]
            pub fn #mut_fn_name(&mut self) -> &mut #r#type2 {
                self.#field_name5.1 = true;
                &mut self.#field_name6.0
            }
        }
    }
}
