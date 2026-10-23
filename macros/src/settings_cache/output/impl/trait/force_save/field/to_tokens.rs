use crate::settings_cache::SettingsCacheOutputForceSaveFnField;
use proc_macro_util::{Generator, ToTokens, to_tokens};

impl<'a> ToTokens for SettingsCacheOutputForceSaveFnField<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        let SettingsCacheOutputForceSaveFnField { name } = self;

        let name2 = name.clone();
        let name3 = name.clone();

        to_tokens! { generator
            self.__write_states.push(unsafe {
                ::colosseum::SettingsGroup::save(
                    &new_settings.#name.0,
                    &self.__path,
                    &self.__logger,
                    &self.__file_io,
                )
            });
            self.#name2 = new_settings.#name3.0.clone();
        }
    }
}
