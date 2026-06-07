use proc_macro_util::{ast::Type, tokens::Identifier};
use std::borrow::Cow;

mod from_input;
mod to_tokens;

/// A getter function for a field in the settings cache output struct.
pub struct SettingsCacheOutputModifiableGetterFn<'a> {
    /// The name of the field to produce a getter function for
    name: Cow<'a, Identifier>,

    /// The type of the field to produce a getter function for
    r#type: Type<'a>,
}
