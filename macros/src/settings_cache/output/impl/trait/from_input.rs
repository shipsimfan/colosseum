use crate::settings_cache::{
    SettingsCacheInput, SettingsCacheOutputLoadFn, SettingsCacheOutputSaveFn,
    SettingsCacheOutputTrait, SettingsCacheOutputTraitModifyField,
};

impl<'a> SettingsCacheOutputTrait<'a> {
    /// Create a new [`SettingsCacheOutputTrait`] from a [`SettingsCacheInput`]
    pub fn from_input(input: &SettingsCacheInput<'a>) -> SettingsCacheOutputTrait<'a> {
        SettingsCacheOutputTrait {
            name: input.name.clone(),
            modifiable_name: input.modifiable_name.clone(),
            load_fn: SettingsCacheOutputLoadFn::from_input(input),
            modify_fields: input
                .fields
                .iter()
                .map(SettingsCacheOutputTraitModifyField::from_input)
                .collect(),
            save_fn: SettingsCacheOutputSaveFn::from_input(input),
        }
    }
}
