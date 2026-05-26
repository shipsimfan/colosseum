use crate::settings_cache::{
    SettingsCacheInput, SettingsCacheOutputStruct, SettingsCacheOutputStructField,
};

impl<'a> SettingsCacheOutputStruct<'a> {
    /// Create a new [`SettingsCacheOutputStruct`] from a [`SettingsCacheInput`]
    pub fn from_input(input: SettingsCacheInput<'a>) -> SettingsCacheOutputStruct<'a> {
        SettingsCacheOutputStruct {
            attributes: input.attributes,
            visibility: input.visibility,
            name: input.name,
            generic_params: input.generic_params,
            where_clause: input.where_clause,
            fields: input
                .fields
                .into_iter()
                .map(SettingsCacheOutputStructField::from_input)
                .collect(),
        }
    }
}
