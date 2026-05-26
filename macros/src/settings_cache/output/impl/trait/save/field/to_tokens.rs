use crate::settings_cache::SettingsCacheOutputSaveFnField;
use proc_macro_util::{Generator, ToTokens, to_tokens};

impl<'a> ToTokens for SettingsCacheOutputSaveFnField<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        let SettingsCacheOutputSaveFnField { name } = self;

        let name2 = name.clone();
        let name3 = name.clone();

        to_tokens! { generator
            unsafe { ::colosseum::settings::SettingsGroup::save(&new_settings.#name, &self.__path, &self.__logger)? };
            self.#name2 = new_settings.#name3.clone();
        }
    }
}
