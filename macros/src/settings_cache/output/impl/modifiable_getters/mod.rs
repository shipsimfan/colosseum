use proc_macro_util::tokens::Identifier;
use std::borrow::Cow;

mod r#fn;

mod from_input;
mod to_tokens;

pub use r#fn::*;

/// The getter functions to produce for a settings cache output struct
pub struct SettingsCacheOutputModifiableGetterFns<'a> {
    /// The name of the struct to produce functions for
    name: Cow<'a, Identifier>,

    /// The functions to produce for the struct
    fns: Vec<SettingsCacheOutputModifiableGetterFn<'a>>,
}
