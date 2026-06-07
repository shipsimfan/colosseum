use crate::settings_cache::SettingsCacheOutputModifiableGetterFns;
use proc_macro_util::{Generator, ToTokens, to_tokens};

impl<'a> ToTokens for SettingsCacheOutputModifiableGetterFns<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        let SettingsCacheOutputModifiableGetterFns { name, fns } = self;

        to_tokens! { generator
            impl #name {
                #fns
            }
        }
    }
}
