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

                fn display_settings(&self) -> &::colosseum::settings::DisplaySettings {
                    &self.display
                }
            }
        }
    }
}
