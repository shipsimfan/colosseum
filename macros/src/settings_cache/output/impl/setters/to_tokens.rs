use crate::settings_cache::SettingsCacheOutputSetterFns;
use proc_macro_util::{Generator, ToTokens, to_tokens};

impl<'a> ToTokens for SettingsCacheOutputSetterFns<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        let SettingsCacheOutputSetterFns { name, fns } = self;

        to_tokens! { generator
            impl #name {
                #fns
            }
        }
    }
}
