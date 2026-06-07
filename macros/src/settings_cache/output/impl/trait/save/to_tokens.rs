use crate::settings_cache::SettingsCacheOutputSaveFn;
use proc_macro_util::{Generator, ToTokens, to_tokens};

impl<'a> ToTokens for SettingsCacheOutputSaveFn<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        let SettingsCacheOutputSaveFn { fields } = self;

        to_tokens! { generator
            fn save(&mut self, new_settings: &Self::Modifiable) {
                if self.is_saving() {
                    panic!("cannot save settings while a save is already in progress");
                }

                #fields
            }
        }
    }
}
