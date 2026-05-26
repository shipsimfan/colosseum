mod r#impl;
mod modifable_struct;
mod r#struct;

mod from_input;
mod to_tokens;

pub use r#impl::*;
pub use modifable_struct::*;
pub use r#struct::*;

/// Generates the tokens to produce a settings cache
pub struct SettingsCacheOutput<'a> {
    /// The struct definition
    r#struct: SettingsCacheOutputStruct<'a>,

    /// The modifiable struct definition
    modifiable_struct: SettingsCacheOutputModifiableStruct<'a>,

    /// The getter functions for the struct
    getters: SettingsCacheOutputGetterFns<'a>,

    /// The trait implementation
    r#trait: SettingsCacheOutputTrait<'a>,

    /// The getter functions for the modifiable struct
    modifiable_getters: SettingsCacheOutputGetterFns<'a>,

    /// The setter functions for the modifiable struct
    setters: SettingsCacheOutputSetterFns<'a>,
}
