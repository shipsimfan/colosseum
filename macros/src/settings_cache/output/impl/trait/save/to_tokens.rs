use crate::settings_cache::SettingsCacheOutputSaveFn;
use proc_macro_util::{Generator, ToTokens, to_tokens};

impl ToTokens for SettingsCacheOutputSaveFn {
    fn to_tokens(self, generator: &mut Generator) {
        to_tokens! { generator
            fn save(&self, path: &::std::path::Path) -> ::colosseum::Result<()> {
                Ok(())
            }
        }
    }
}
