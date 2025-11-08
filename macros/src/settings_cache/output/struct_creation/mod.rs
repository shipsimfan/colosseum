use proc_macro_util::tokens::Identifier;

mod from_input;
mod to_tokens;

/// Produces the tokens to generate the creation of a single field in the struct
pub struct SettingsCacheStructCreation {
    /// The name of the field to insert
    field_name: Identifier,
}
