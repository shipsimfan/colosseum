use crate::settings_cache::{
    SettingsCacheInput, SettingsCacheOutputSaveFn, SettingsCacheOutputSaveFnField,
};

impl<'a> SettingsCacheOutputSaveFn<'a> {
    /// Create a new [`SettingsCacheOutputSaveFn`] from a [`SettingsCacheInput`]
    pub fn from_input(input: &SettingsCacheInput<'a>) -> SettingsCacheOutputSaveFn<'a> {
        SettingsCacheOutputSaveFn {
            fields: input
                .fields
                .iter()
                .map(SettingsCacheOutputSaveFnField::from_input)
                .collect(),
        }
    }
}
