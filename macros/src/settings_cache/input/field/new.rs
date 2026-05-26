use crate::settings_cache::SettingsCacheInputField;
use proc_macro_util::{
    Result,
    ast::items::{StructField, StructFields},
};

impl<'a> SettingsCacheInputField<'a> {
    /// Create a new set of [`SettingsCacheInputField`]s from the [`StructFields`]
    pub fn new_set(fields: Option<StructFields<'a>>) -> Result<Vec<SettingsCacheInputField<'a>>> {
        let fields = match fields {
            Some(fields) => fields,
            None => return Ok(Vec::new()),
        };

        let mut result = Vec::with_capacity(fields.remaining.len() + 1);

        result.push(SettingsCacheInputField::new(fields.first)?);
        for (_, field) in fields.remaining {
            result.push(SettingsCacheInputField::new(field)?);
        }

        Ok(result)
    }

    /// Create a new [`SettingsCacheInputField`] from a [`StructField`]
    pub fn new(field: StructField<'a>) -> Result<SettingsCacheInputField<'a>> {
        Ok(SettingsCacheInputField {
            attributes: field.attributes,
            name: field.name,
            r#type: field.r#type,
        })
    }
}
