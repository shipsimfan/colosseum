use proc_macro_util::{ast::Type, tokens::Identifier};

mod from_input;
mod to_tokens;

/// Produces the tokens to generate the loading call for a field
pub struct SettingsCacheLoad<'a> {
    /// The name of the field to load
    field_name: Identifier,

    /// The type of the field to load
    field_type: Type<'a>,
}
