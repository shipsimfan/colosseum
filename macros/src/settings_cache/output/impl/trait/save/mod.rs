mod field;

mod from_input;
mod to_tokens;

pub use field::*;

/// The function that will be called to save the settings cache output
pub struct SettingsCacheOutputSaveFn<'a> {
    /// The fields to create for the save function
    fields: Vec<SettingsCacheOutputSaveFnField<'a>>,
}
