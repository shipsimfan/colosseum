use crate::settings_cache::SettingsCacheOutputStructField;
use proc_macro_util::{Generator, ToTokens, to_tokens};

impl<'a> ToTokens for SettingsCacheOutputStructField<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        let SettingsCacheOutputStructField {
            attributes,
            name,
            r#type,
        } = self;

        to_tokens! { generator
            #attributes
            #name: #r#type,
        }
    }
}
