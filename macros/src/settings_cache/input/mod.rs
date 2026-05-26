use proc_macro_util::{
    ast::{OuterAttribute, Visibility},
    tokens::Identifier,
};
use std::borrow::Cow;

mod new;

/// The input to the settings cache macro
pub struct SettingsCacheInput<'a> {
    /// The outer attributes on the struct
    pub attributes: Vec<OuterAttribute<'a>>,

    /// The visibility of the struct
    pub visibility: Option<Visibility<'a>>,

    /// The name of the struct
    pub name: Cow<'a, Identifier>,
}
