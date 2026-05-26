use crate::settings_cache::SettingsCacheOutputTrait;
use proc_macro_util::{Generator, ToTokens, to_tokens};

impl<'a> ToTokens for SettingsCacheOutputTrait<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        let SettingsCacheOutputTrait {
            name,
            load_fn,
            save_fn,
        } = self;

        to_tokens! { generator
            impl ::colosseum::settings::SettingsCache for #name {
                #load_fn

                #save_fn
            }
        }
    }
}
