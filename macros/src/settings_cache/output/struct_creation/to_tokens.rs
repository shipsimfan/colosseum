use crate::settings_cache::output::SettingsCacheStructCreation;
use proc_macro_util::{Generator, ToTokens, to_tokens};

impl ToTokens for SettingsCacheStructCreation {
    fn to_tokens(self, generator: &mut Generator) {
        let SettingsCacheStructCreation { field_name } = self;

        to_tokens! { generator
            #field_name,
        }
    }
}
