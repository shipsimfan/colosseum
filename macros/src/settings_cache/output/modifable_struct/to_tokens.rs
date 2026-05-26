use crate::settings_cache::SettingsCacheOutputModifiableStruct;
use proc_macro_util::{Generator, ToTokens, to_tokens};

impl<'a> ToTokens for SettingsCacheOutputModifiableStruct<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        let SettingsCacheOutputModifiableStruct {
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
            }
        }
    }
}
