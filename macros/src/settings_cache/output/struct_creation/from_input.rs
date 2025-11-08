use crate::settings_cache::output::SettingsCacheStructCreation;
use proc_macro_util::ast::items::StructField;

impl SettingsCacheStructCreation {
    /// Create a new [`SettingsCacheStructCreation`] from a `field`
    pub fn from_input(field: &StructField) -> Self {
        SettingsCacheStructCreation {
            field_name: field.name.as_ref().clone(),
        }
    }
}
