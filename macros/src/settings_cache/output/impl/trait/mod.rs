use proc_macro_util::tokens::Identifier;
use std::borrow::Cow;

mod load;
mod save;

mod from_input;
mod to_tokens;

pub use load::*;
pub use save::*;

/// The implementation of the trait for the settings cache output
pub struct SettingsCacheOutputTrait<'a> {
    /// The name of the struct
    name: Cow<'a, Identifier>,

    /// The function that will be called to load the settings cache output
    load_fn: SettingsCacheOutputLoadFn,

    /// The function that will be called to save the settings cache output
    save_fn: SettingsCacheOutputSaveFn,
}
