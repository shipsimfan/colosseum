use crate::settings_cache::SettingsCacheOutputLoadFn;
use proc_macro_util::{Generator, ToTokens, to_tokens};

impl<'a> ToTokens for SettingsCacheOutputLoadFn<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        let SettingsCacheOutputLoadFn { fields } = self;

        to_tokens! { generator
            fn load(
                path: &::std::path::Path,
                logger: ::colosseum::logging::Logger,
                file_io: &::colosseum::file_io::FileIo
            ) -> ::colosseum::Result<Self> {
                Ok(Self {
                    #fields

                    __logger: logger,
                    __path: path.to_path_buf(),
                    __file_io: file_io.clone(),
                    __write_states: Vec::new(),
                })
            }
        }
    }
}
