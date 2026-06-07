use crate::settings_cache::SettingsCacheOutputStruct;
use proc_macro_util::{Generator, ToTokens, to_tokens};

impl<'a> ToTokens for SettingsCacheOutputStruct<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        let SettingsCacheOutputStruct {
            attributes,
            visibility,
            name,
            generic_params,
            where_clause,
            fields,
        } = self;

        to_tokens! { generator
            #attributes
            #visibility struct #name #generic_params #where_clause {
                #fields

                __logger: ::colosseum::logging::Logger,
                __path: ::std::path::PathBuf,
                __file_io: ::colosseum::file_io::FileIo,
                __write_states: ::std::vec::Vec<::colosseum::file_io::WriteFullFile>,
            }
        }
    }
}
