use crate::settings_cache::SettingsCacheOutputTrait;
use proc_macro_util::{Generator, ToTokens, to_tokens};

impl<'a> ToTokens for SettingsCacheOutputTrait<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        let SettingsCacheOutputTrait {
            name,
            modifiable_name,
            load_fn,
            modify_fields,
            save_fn,
        } = self;

        let modifiable_name2 = modifiable_name.clone();

        to_tokens! { generator
            impl ::colosseum::settings::SettingsCache for #name {
                type Modifiable = #modifiable_name;

                #load_fn

                fn begin_modify(&self) -> Self::Modifiable {
                    Self::Modifiable {
                        #modify_fields
                    }
                }

                #save_fn

                fn is_saving(&mut self) -> bool {
                    let mut i = 0;
                    while i < self.__write_states.len() {
                        if self.__write_states[i].is_complete() {
                            self.__write_states.swap_remove(i);
                        } else {
                            i += 1;
                        }
                    }

                    !self.__write_states.is_empty()
                }

                fn display_settings(&self) -> &::colosseum::settings::DisplaySettings {
                    &self.display
                }
            }

            impl ::colosseum::settings::ModifiableSettingsCache for #modifiable_name2 {
                fn display_settings_mut(&mut self) -> &mut ::colosseum::settings::DisplaySettings {
                    self.display_mut()
                }
            }
        }
    }
}
