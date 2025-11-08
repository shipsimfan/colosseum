use crate::settings_cache::output::SettingsCacheSave;
use proc_macro_util::{Generator, ToTokens, to_tokens};

impl ToTokens for SettingsCacheSave {
    fn to_tokens(self, generator: &mut Generator) {
        let SettingsCacheSave { field_name } = self;

        to_tokens! { generator
            unsafe { #field_name.save(path) }?;
        }
    }
}
