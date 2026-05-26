use crate::settings_cache::{
    SettingsCacheInput, SettingsCacheOutputLoadFn, SettingsCacheOutputLoadFnField,
};

impl<'a> SettingsCacheOutputLoadFn<'a> {
    /// Create a new [`SettingsCacheOutputLoadFn`] from a [`SettingsCacheInput`]
    pub fn from_input(input: &SettingsCacheInput<'a>) -> SettingsCacheOutputLoadFn<'a> {
        SettingsCacheOutputLoadFn {
            fields: input
                .fields
                .iter()
                .map(SettingsCacheOutputLoadFnField::from_input)
                .collect(),
        }
    }
}
