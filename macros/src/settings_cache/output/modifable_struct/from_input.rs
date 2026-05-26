use crate::settings_cache::{
    SettingsCacheInput, SettingsCacheOutputModifiableStruct,
    SettingsCacheOutputModifiableStructField,
};

impl<'a> SettingsCacheOutputModifiableStruct<'a> {
    /// Create a new [`SettingsCacheOutputModifiableStruct`] from a [`SettingsCacheInput`]
    pub fn from_input(input: &SettingsCacheInput<'a>) -> SettingsCacheOutputModifiableStruct<'a> {
        SettingsCacheOutputModifiableStruct {
            attributes: input.attributes.clone(),
            visibility: input.visibility.clone(),
            name: input.modifiable_name.clone(),
            generic_params: input.generic_params.clone(),
            where_clause: input.where_clause.clone(),
            fields: input
                .fields
                .iter()
                .map(SettingsCacheOutputModifiableStructField::from_input)
                .collect(),
        }
    }
}
