use proc_macro_util::tokens::Identifier;
use std::borrow::Cow;

mod from_input;
mod to_tokens;

/// A field to create the modifiable version of the settings cache output struct
pub struct SettingsCacheOutputTraitModifyField<'a> {
    /// The name of the field to produce
    name: Cow<'a, Identifier>,
}
