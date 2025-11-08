use crate::settings_cache::output::SettingsCacheLoad;
use proc_macro_util::ast::items::StructField;

impl<'a> SettingsCacheLoad<'a> {
    /// Create a new [`SettingsCacheLoad`] from a `field`
    pub fn from_input(field: &StructField<'a>) -> Self {
        SettingsCacheLoad {
            field_name: field.name.as_ref().clone(),
            field_type: field.r#type.clone(),
        }
    }
}
