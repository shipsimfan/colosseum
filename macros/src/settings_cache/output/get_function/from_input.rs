use crate::settings_cache::output::SettingsCacheGetFunction;
use proc_macro_util::ast::items::StructField;

impl<'a> SettingsCacheGetFunction<'a> {
    /// Create a new [`SettingsCacheGetFunction`] from a `field`
    pub fn from_input(field: &StructField<'a>) -> Self {
        SettingsCacheGetFunction {
            field_name: field.name.as_ref().clone(),
            field_type: field.r#type.clone(),
        }
    }
}
