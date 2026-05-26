use crate::settings_cache::{SettingsCacheInputField, SettingsCacheOutputGetterFn};

impl<'a> SettingsCacheOutputGetterFn<'a> {
    /// Create a new [`SettingsCacheOutputGetterFn`] from a [`SettingsCacheInputField`]
    pub fn from_input(input: &SettingsCacheInputField<'a>) -> SettingsCacheOutputGetterFn<'a> {
        SettingsCacheOutputGetterFn {
            name: input.name.clone(),
            r#type: input.r#type.clone(),
        }
    }
}
