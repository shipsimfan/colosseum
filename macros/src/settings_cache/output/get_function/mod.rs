use proc_macro_util::{ast::Type, tokens::Identifier};

mod from_input;
mod to_tokens;

/// Produces the tokens to generate a get function for a single field
pub struct SettingsCacheGetFunction<'a> {
    /// The name of the field to produce the function for
    field_name: Identifier,

    /// The type of the field
    field_type: Type<'a>,
}
