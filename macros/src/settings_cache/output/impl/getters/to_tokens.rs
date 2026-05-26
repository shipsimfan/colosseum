use crate::settings_cache::SettingsCacheOutputGetterFns;
use proc_macro_util::{Generator, ToTokens, to_tokens};

impl<'a> ToTokens for SettingsCacheOutputGetterFns<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        let SettingsCacheOutputGetterFns { name, fns } = self;

        to_tokens! { generator
            impl #name {
                #fns
            }
        }
    }
}
