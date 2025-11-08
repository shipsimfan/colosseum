use crate::settings_cache::output::SettingsCacheLoad;
use proc_macro_util::{Generator, ToTokens, to_tokens};

impl<'a> ToTokens for SettingsCacheLoad<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        let SettingsCacheLoad {
            field_name,
            field_type,
        } = self;

        to_tokens! { generator
            let #field_name: #field_type =
                unsafe { ::colosseum::settings::SettingsGroup::load(path)? };
        }
    }
}
