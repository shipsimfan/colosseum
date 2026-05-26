use crate::settings_cache::{SettingsCacheInputField, SettingsCacheOutputStructField};

impl<'a> SettingsCacheOutputStructField<'a> {
    /// Create a new [`SettingsCacheOutputStructField`] from a [`SettingsCacheInput`]
    pub fn from_input(input: SettingsCacheInputField<'a>) -> SettingsCacheOutputStructField<'a> {
        SettingsCacheOutputStructField {
            attributes: input.attributes,
            name: input.name,
            r#type: input.r#type,
        }
    }
}
