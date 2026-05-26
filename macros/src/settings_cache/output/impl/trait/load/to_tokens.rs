use crate::settings_cache::SettingsCacheOutputLoadFn;
use proc_macro_util::{Generator, ToTokens, to_tokens};

impl ToTokens for SettingsCacheOutputLoadFn {
    fn to_tokens(self, generator: &mut Generator) {
        to_tokens! { generator
            fn load(
                path: &::std::path::Path,
                logger: ::colosseum::logging::Logger
            ) -> ::colosseum::Result<Self> {
                Ok(Self {})
            }
        }
    }
}
