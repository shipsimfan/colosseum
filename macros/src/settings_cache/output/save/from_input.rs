use crate::settings_cache::output::SettingsCacheSave;
use proc_macro_util::ast::items::StructField;

impl SettingsCacheSave {
    /// Create a new [`SettingsCacheSave`] from a `field`
    pub fn from_input(field: &StructField) -> Self {
        SettingsCacheSave {
            field_name: field.name.as_ref().clone(),
        }
    }
}
