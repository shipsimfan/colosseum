use crate::settings_cache::{
    SettingsCacheInput, SettingsCacheOutput, SettingsCacheOutputStruct, SettingsCacheOutputTrait,
};

impl<'a> SettingsCacheOutput<'a> {
    /// Create a new [`SettingsCacheOutput`] from a [`SettingsCacheInput`]
    pub fn from_input(input: SettingsCacheInput<'a>) -> SettingsCacheOutput<'a> {
        SettingsCacheOutput {
            r#trait: SettingsCacheOutputTrait::from_input(&input),
            r#struct: SettingsCacheOutputStruct::from_input(input),
        }
    }
}
