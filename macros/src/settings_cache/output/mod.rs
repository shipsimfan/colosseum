use std::borrow::Cow;
use get_function::SettingsCacheGetFunction;
use load::SettingsCacheLoad;
use proc_macro_util::{ast::Item, tokens::Identifier};
use save::SettingsCacheSave;
use set_function::SettingsCacheSetFunction;
use struct_creation::SettingsCacheStructCreation;

mod get_function;
mod load;
mod save;
mod set_function;
mod struct_creation;

mod from_input;
mod to_tokens;

/// Generates the tokens to produce a settings cache
pub struct SettingsCacheOutput<'a> {
    /// The struct definition
    item: Item<'a>,

    /// The name of the struct
    name: Cow<'a, Identifier>,

    /// The get functions for this cache
    get_functions: Vec<SettingsCacheGetFunction<'a>>,

    /// The set functions for this cache
    set_functions: Vec<SettingsCacheSetFunction<'a>>,

    /// The calls to load the settings groups
    loads: Vec<SettingsCacheLoad<'a>>,

    /// The calls to save the settings groups
    saves: Vec<SettingsCacheSave>,

    struct_creation: Vec<SettingsCacheStructCreation>,
}
