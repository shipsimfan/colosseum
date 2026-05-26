use crate::settings_cache::SettingsCacheOutputStruct;
use proc_macro_util::{Generator, ToTokens, to_tokens};

impl<'a> ToTokens for SettingsCacheOutputStruct<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        let SettingsCacheOutputStruct {
            attributes,
            visibility,
            name,
        } = self;

        to_tokens! { generator
            #attributes
            #visibility struct #name {

            }
        }
    }
}
