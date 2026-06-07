use crate::settings_cache::SettingsCacheOutputModifiableGetterFn;
use proc_macro_util::{Generator, ToTokens, to_tokens};

impl<'a> ToTokens for SettingsCacheOutputModifiableGetterFn<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        let SettingsCacheOutputModifiableGetterFn { name, r#type } = self;

        let name2 = name.clone();
        let name3 = name.clone();

        to_tokens! { generator
            #[doc = ::std::concat!("Get a reference to the `", ::std::stringify!(#name), "` settings group")]
            pub fn #name2(&self) -> &#r#type {
                &self.#name3.0
            }
        }
    }
}
