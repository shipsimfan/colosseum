mod field;

mod from_input;
mod to_tokens;

pub use field::*;

/// The function that will be called to force save the settings cache output
pub struct SettingsCacheOutputForceSaveFn<'a> {
    /// The fields to create for the save function
    fields: Vec<SettingsCacheOutputForceSaveFnField<'a>>,
}
