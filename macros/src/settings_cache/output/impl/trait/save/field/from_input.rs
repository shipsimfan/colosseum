use crate::settings_cache::{SettingsCacheInputField, SettingsCacheOutputSaveFnField};

impl<'a> SettingsCacheOutputSaveFnField<'a> {
    /// Create a new [`SettingsCacheOutputSaveFnField`] from a [`SettingsCacheInputField`]
    pub fn from_input(input: &SettingsCacheInputField<'a>) -> SettingsCacheOutputSaveFnField<'a> {
        SettingsCacheOutputSaveFnField {
            name: input.name.clone(),
        }
    }
}
