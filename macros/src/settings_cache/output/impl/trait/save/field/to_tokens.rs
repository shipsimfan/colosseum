use crate::settings_cache::SettingsCacheOutputSaveFnField;
use proc_macro_util::{Generator, ToTokens, to_tokens};

impl<'a> ToTokens for SettingsCacheOutputSaveFnField<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        let SettingsCacheOutputSaveFnField { name } = self;

        let name2 = name.clone();
        let name3 = name.clone();
        let name4 = name.clone();

        to_tokens! { generator
            if new_settings.#name.1 {
                self.__write_states.push(unsafe {
                    ::colosseum::settings::SettingsGroup::save(
                        &new_settings.#name2.0,
                        &self.__path,
                        &self.__logger,
                        &self.__file_io,
                    )
                });
                self.#name3 = new_settings.#name4.0.clone();
            }
        }
    }
}
