use proc_macro_util::{
    ast::{OuterAttribute, Type},
    tokens::Identifier,
};
use std::borrow::Cow;

mod new;

/// A field in the input to the settings cache macro
pub struct SettingsCacheInputField<'a> {
    /// The outer attributes on the field
    pub attributes: Vec<OuterAttribute<'a>>,

    /// The name of the field
    pub name: Cow<'a, Identifier>,

    /// The type of the field
    pub r#type: Type<'a>,
}
