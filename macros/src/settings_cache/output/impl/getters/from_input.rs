use crate::settings_cache::{
    SettingsCacheInputField, SettingsCacheOutputGetterFn, SettingsCacheOutputGetterFns,
};
use proc_macro_util::tokens::Identifier;
use std::borrow::Cow;

impl<'a> SettingsCacheOutputGetterFns<'a> {
    /// Create a new [`SettingsCacheOutputGetterFns`] from [`SettingsCacheInputField`]s
    pub fn from_input<I: Into<Cow<'a, Identifier>>>(
        name: I,
        fields: &[SettingsCacheInputField<'a>],
    ) -> SettingsCacheOutputGetterFns<'a> {
        SettingsCacheOutputGetterFns {
            name: name.into(),
            fns: fields
                .iter()
                .map(SettingsCacheOutputGetterFn::from_input)
                .collect(),
        }
    }
}
