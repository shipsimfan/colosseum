use crate::settings_cache::{
    SettingsCacheInputField, SettingsCacheOutputModifiableGetterFn,
    SettingsCacheOutputModifiableGetterFns,
};
use proc_macro_util::tokens::Identifier;
use std::borrow::Cow;

impl<'a> SettingsCacheOutputModifiableGetterFns<'a> {
    /// Create a new [`SettingsCacheOutputModifiableGetterFns`] from [`SettingsCacheInputField`]s
    pub fn from_input<I: Into<Cow<'a, Identifier>>>(
        name: I,
        fields: &[SettingsCacheInputField<'a>],
    ) -> SettingsCacheOutputModifiableGetterFns<'a> {
        SettingsCacheOutputModifiableGetterFns {
            name: name.into(),
            fns: fields
                .iter()
                .map(SettingsCacheOutputModifiableGetterFn::from_input)
                .collect(),
        }
    }
}
