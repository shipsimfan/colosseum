use crate::settings_cache::{
    SettingsCacheInput, SettingsCacheOutputSetterFn, SettingsCacheOutputSetterFns,
};

impl<'a> SettingsCacheOutputSetterFns<'a> {
    /// Create a new [`SettingsCacheOutputSetterFns`] from a [`SettingsCacheInput`]
    pub fn from_input(input: &SettingsCacheInput<'a>) -> SettingsCacheOutputSetterFns<'a> {
        SettingsCacheOutputSetterFns {
            name: input.modifiable_name.clone(),
            fns: input
                .fields
                .iter()
                .map(SettingsCacheOutputSetterFn::from_input)
                .collect(),
        }
    }
}
