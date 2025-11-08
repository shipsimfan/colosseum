use proc_macro_util::{ast::Type, tokens::Identifier};

mod from_input;
mod to_tokens;

/// Produces the tokens to generate a set function for a single field
pub struct SettingsCacheSetFunction<'a> {
    /// The name of the function to produce
    function_name: Identifier,

    /// The name of the field to produce the function for
    field_name: Identifier,

    /// The type of the field
    field_type: Type<'a>,
}
