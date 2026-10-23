use proc_macro_util::tokens::Identifier;
use std::borrow::Cow;

mod force_save;
mod load;
mod modify_field;
mod save;

mod from_input;
mod to_tokens;

pub use force_save::*;
pub use load::*;
pub use modify_field::*;
pub use save::*;

/// The implementation of the trait for the settings cache output
pub struct SettingsCacheOutputTrait<'a> {
    /// The name of the struct
    name: Cow<'a, Identifier>,

    /// The name of the modifiable struct
    modifiable_name: Identifier,

    /// The function that will be called to load the settings cache output
    load_fn: SettingsCacheOutputLoadFn<'a>,

    /// The fields to produce for the modifiable version of the settings cache output struct
    modify_fields: Vec<SettingsCacheOutputTraitModifyField<'a>>,

    /// The function that will be called to save the settings cache output
    save_fn: SettingsCacheOutputSaveFn<'a>,

    /// The function that will be called to force save the settings cache output
    force_save_fn: SettingsCacheOutputForceSaveFn<'a>,
}
