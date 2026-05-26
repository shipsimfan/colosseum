use crate::settings_cache::SettingsCacheOutput;
use proc_macro_util::{Generator, ToTokens, to_tokens};

impl<'a> ToTokens for SettingsCacheOutput<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        let SettingsCacheOutput {
            r#struct,
            modifiable_struct,
            getters,
            r#trait,
            modifiable_getters,
            setters,
        } = self;

        to_tokens! { generator
            #r#struct

            #modifiable_struct

            #getters

            #r#trait

            #modifiable_getters

            #setters
        }
    }
}
