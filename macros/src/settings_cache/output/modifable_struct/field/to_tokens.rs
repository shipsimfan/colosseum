use crate::settings_cache::SettingsCacheOutputModifiableStructField;
use proc_macro_util::{Generator, ToTokens, to_tokens};

impl<'a> ToTokens for SettingsCacheOutputModifiableStructField<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        let SettingsCacheOutputModifiableStructField {
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
