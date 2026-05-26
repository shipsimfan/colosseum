use crate::settings_cache::{SettingsCacheInputField, SettingsCacheOutputModifiableStructField};

impl<'a> SettingsCacheOutputModifiableStructField<'a> {
    /// Create a new [`SettingsCacheOutputModifiableStructField`] from a [`SettingsCacheInputField`]
    pub fn from_input(
        input: &SettingsCacheInputField<'a>,
    ) -> SettingsCacheOutputModifiableStructField<'a> {
        SettingsCacheOutputModifiableStructField {
            attributes: input.attributes.clone(),
            name: input.name.clone(),
            r#type: input.r#type.clone(),
        }
    }
}
