use proc_macro_util::tokens::Identifier;
use std::borrow::Cow;

mod from_input;
mod to_tokens;

/// Create a field for the load function
pub struct SettingsCacheOutputLoadFnField<'a> {
    /// The name of the field to create
    name: Cow<'a, Identifier>,
}
