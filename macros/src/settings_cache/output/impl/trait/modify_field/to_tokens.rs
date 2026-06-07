use crate::settings_cache::SettingsCacheOutputTraitModifyField;
use proc_macro_util::{Generator, ToTokens, to_tokens};

impl<'a> ToTokens for SettingsCacheOutputTraitModifyField<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        let SettingsCacheOutputTraitModifyField { name } = self;

        let name2 = name.clone();

        to_tokens! { generator
            #name: (self.#name2.clone(), false),
        }
    }
}
