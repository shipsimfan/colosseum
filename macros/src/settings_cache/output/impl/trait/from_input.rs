use crate::settings_cache::{
    SettingsCacheInput, SettingsCacheOutputLoadFn, SettingsCacheOutputSaveFn,
    SettingsCacheOutputTrait,
};

impl<'a> SettingsCacheOutputTrait<'a> {
    /// Create a new [`SettingsCacheOutputTrait`] from a [`SettingsCacheInput`]
    pub fn from_input(input: &SettingsCacheInput<'a>) -> SettingsCacheOutputTrait<'a> {
        SettingsCacheOutputTrait {
            name: input.name.clone(),
            load_fn: SettingsCacheOutputLoadFn::from_input(input),
            save_fn: SettingsCacheOutputSaveFn::from_input(input),
        }
    }
}
