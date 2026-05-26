use crate::settings_cache::SettingsCacheOutputLoadFnField;
use proc_macro_util::{Generator, ToTokens, to_tokens};

impl<'a> ToTokens for SettingsCacheOutputLoadFnField<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        let SettingsCacheOutputLoadFnField { name } = self;

        to_tokens! { generator
            #name: unsafe { ::colosseum::settings::SettingsGroup::load(path, &logger)? },
        }
    }
}
