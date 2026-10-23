use proc_macro_util::tokens::Identifier;
use std::borrow::Cow;

mod from_input;
mod to_tokens;

/// Create a field for the force save function
pub struct SettingsCacheOutputForceSaveFnField<'a> {
    /// The name of the field to save
    name: Cow<'a, Identifier>,
}
