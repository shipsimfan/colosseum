use crate::settings_cache::SettingsCacheOutput;
use proc_macro_util::{Generator, ToTokens, to_tokens};

impl<'a> ToTokens for SettingsCacheOutput<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        let SettingsCacheOutput {
            item,
            name,
            get_functions,
            set_functions,
            loads,
            saves,
            struct_creation,
        } = self;

        let name2 = name.clone();
        let name3 = name.clone();

        to_tokens! { generator
            #item

            impl #name {
                #get_functions

                #set_functions
            }

            impl ::colosseum::settings::SettingsCache for #name2 {
                fn load(path: &::std::path::Path) -> ::colosseum::Result<Self> {
                    #loads

                    #saves

                    Ok(#name3 {
                        #struct_creation
                    })
                }

                fn graphics_settings(&self) -> &::colosseum::graphics::GraphicsSettings {
                    &self.graphics_settings
                }
            }
        }
    }
}
