mod r#impl;
mod r#struct;

mod from_input;
mod to_tokens;

pub use r#impl::*;
pub use r#struct::*;

/// Generates the tokens to produce a settings cache
pub struct SettingsCacheOutput<'a> {
    /// The struct definition
    r#struct: SettingsCacheOutputStruct<'a>,

    /// The trait implementation
    r#trait: SettingsCacheOutputTrait<'a>,
}
