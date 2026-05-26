use proc_macro_util::tokens::Identifier;

mod r#fn;

mod from_input;
mod to_tokens;

pub use r#fn::*;

/// The setter functions to produce for a settings cache output struct
pub struct SettingsCacheOutputSetterFns<'a> {
    /// The name of the struct to produce functions for
    name: Identifier,

    /// The functions to produce for the struct
    fns: Vec<SettingsCacheOutputSetterFn<'a>>,
}
