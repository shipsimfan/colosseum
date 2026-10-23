use crate::settings_cache::{SettingsCacheInputField, SettingsCacheOutputForceSaveFnField};

impl<'a> SettingsCacheOutputForceSaveFnField<'a> {
    /// Create a new [`SettingsCacheOutputForceSaveFnField`] from a [`SettingsCacheInputField`]
    pub fn from_input(
        input: &SettingsCacheInputField<'a>,
    ) -> SettingsCacheOutputForceSaveFnField<'a> {
        SettingsCacheOutputForceSaveFnField {
            name: input.name.clone(),
        }
    }
}
