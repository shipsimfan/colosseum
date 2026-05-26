use proc_macro_util::tokens::Identifier;
use std::borrow::Cow;

mod from_input;
mod to_tokens;

/// Create a field for the save function
pub struct SettingsCacheOutputSaveFnField<'a> {
    /// The name of the field to save
    name: Cow<'a, Identifier>,
}
