use crate::settings_cache::{SettingsCacheInput, SettingsCacheOutputLoadFn};

impl SettingsCacheOutputLoadFn {
    /// Create a new [`SettingsCacheOutputLoadFn`] from a [`SettingsCacheInput`]
    pub fn from_input(input: &SettingsCacheInput) -> SettingsCacheOutputLoadFn {
        SettingsCacheOutputLoadFn {}
    }
}
