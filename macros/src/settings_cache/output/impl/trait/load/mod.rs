mod field;

mod from_input;
mod to_tokens;

pub use field::*;

/// The function that will be called to load the settings cache output
pub struct SettingsCacheOutputLoadFn<'a> {
    /// The fields to create for the load function
    fields: Vec<SettingsCacheOutputLoadFnField<'a>>,
}
