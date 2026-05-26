use crate::settings_cache::{SettingsCacheInput, SettingsCacheOutputSaveFn};

impl SettingsCacheOutputSaveFn {
    /// Create a new [`SettingsCacheOutputSaveFn`] from a [`SettingsCacheInput`]
    pub fn from_input(input: &SettingsCacheInput) -> SettingsCacheOutputSaveFn {
        SettingsCacheOutputSaveFn {}
    }
}
