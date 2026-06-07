use std::borrow::Cow;

use proc_macro_util::{ast::Type, tokens::Identifier};

mod from_input;
mod to_tokens;

/// A setter function for a field in the settings cache output struct.
pub struct SettingsCacheOutputSetterFn<'a> {
    /// The name of the setter function to produce
    fn_name: Identifier,

    /// The name of the mutable getter function to produce
    mut_fn_name: Identifier,

    /// The name of the field to produce a setter function for
    field_name: Cow<'a, Identifier>,

    /// The type of the field to produce a setter function for
    r#type: Type<'a>,
}
