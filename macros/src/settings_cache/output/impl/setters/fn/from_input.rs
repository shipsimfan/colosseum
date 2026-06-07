use proc_macro_util::tokens::Identifier;

use crate::settings_cache::{SettingsCacheInputField, SettingsCacheOutputSetterFn};

impl<'a> SettingsCacheOutputSetterFn<'a> {
    /// Create a new [`SettingsCacheOutputSetterFn`] from a [`SettingsCacheInputField`]
    pub fn from_input(input: &SettingsCacheInputField<'a>) -> SettingsCacheOutputSetterFn<'a> {
        SettingsCacheOutputSetterFn {
            fn_name: Identifier::new(&format!("set_{}", input.name)),
            mut_fn_name: Identifier::new(&format!("{}_mut", input.name)),
            field_name: input.name.clone(),
            r#type: input.r#type.clone(),
        }
    }
}
