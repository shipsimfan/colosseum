use crate::settings_cache::{SettingsCacheInputField, SettingsCacheOutputLoadFnField};

impl<'a> SettingsCacheOutputLoadFnField<'a> {
    /// Create a new [`SettingsCacheOutputLoadFnFieldCreate`] from a [`SettingsCacheInputField`]
    pub fn from_input(input: &SettingsCacheInputField<'a>) -> SettingsCacheOutputLoadFnField<'a> {
        SettingsCacheOutputLoadFnField {
            name: input.name.clone(),
        }
    }
}
