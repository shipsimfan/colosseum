use crate::settings_cache::output::SettingsCacheSetFunction;
use proc_macro_util::{ast::items::StructField, tokens::Identifier};

impl<'a> SettingsCacheSetFunction<'a> {
    /// Create a new [`SettingsCacheSetFunction`] from a `field`
    pub fn from_input(field: &StructField<'a>) -> Self {
        let function_name = Identifier::new(&format!("set_{}", field.name));

        SettingsCacheSetFunction {
            function_name,
            field_name: field.name.as_ref().clone(),
            field_type: field.r#type.clone(),
        }
    }
}
