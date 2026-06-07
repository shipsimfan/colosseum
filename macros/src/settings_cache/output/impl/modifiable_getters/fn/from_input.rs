use crate::settings_cache::{SettingsCacheInputField, SettingsCacheOutputModifiableGetterFn};

impl<'a> SettingsCacheOutputModifiableGetterFn<'a> {
    /// Create a new [`SettingsCacheOutputModifiableGetterFn`] from a [`SettingsCacheInputField`]
    pub fn from_input(
        input: &SettingsCacheInputField<'a>,
    ) -> SettingsCacheOutputModifiableGetterFn<'a> {
        SettingsCacheOutputModifiableGetterFn {
            name: input.name.clone(),
            r#type: input.r#type.clone(),
        }
    }
}
