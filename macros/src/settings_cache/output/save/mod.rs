use proc_macro_util::tokens::Identifier;

mod from_input;
mod to_tokens;

/// Produces the tokens to generate the saving call for a field
pub struct SettingsCacheSave {
    /// The name of the field to save
    field_name: Identifier,
}
