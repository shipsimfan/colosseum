use crate::settings_cache::{SettingsCacheInputField, SettingsCacheOutputTraitModifyField};

impl<'a> SettingsCacheOutputTraitModifyField<'a> {
    /// Create a new [`SettingsCacheOutputTraitModifyField`] from a [`SettingsCacheInputField`]
    pub fn from_input(
        input: &SettingsCacheInputField<'a>,
    ) -> SettingsCacheOutputTraitModifyField<'a> {
        SettingsCacheOutputTraitModifyField {
            name: input.name.clone(),
        }
    }
}
