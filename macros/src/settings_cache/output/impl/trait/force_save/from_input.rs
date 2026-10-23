use crate::settings_cache::{
    SettingsCacheInput, SettingsCacheOutputForceSaveFn, SettingsCacheOutputForceSaveFnField,
};

impl<'a> SettingsCacheOutputForceSaveFn<'a> {
    /// Create a new [`SettingsCacheOutputForceSaveFn`] from a [`SettingsCacheInput`]
    pub fn from_input(input: &SettingsCacheInput<'a>) -> SettingsCacheOutputForceSaveFn<'a> {
        SettingsCacheOutputForceSaveFn {
            fields: input
                .fields
                .iter()
                .map(SettingsCacheOutputForceSaveFnField::from_input)
                .collect(),
        }
    }
}
