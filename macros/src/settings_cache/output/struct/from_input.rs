use crate::settings_cache::{SettingsCacheInput, SettingsCacheOutputStruct};

impl<'a> SettingsCacheOutputStruct<'a> {
    /// Create a new [`SettingsCacheOutputStruct`] from a [`SettingsCacheInput`]
    pub fn from_input(input: SettingsCacheInput<'a>) -> SettingsCacheOutputStruct<'a> {
        SettingsCacheOutputStruct {
            attributes: input.attributes,
            visibility: input.visibility,
            name: input.name,
        }
    }
}
