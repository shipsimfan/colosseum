use proc_macro_util::{
    ast::{GenericParams, OuterAttribute, Visibility, WhereClause},
    tokens::Identifier,
};
use std::borrow::Cow;

mod field;

mod new;

pub use field::*;

/// The input to the settings cache macro
pub struct SettingsCacheInput<'a> {
    /// The outer attributes on the struct
    pub attributes: Vec<OuterAttribute<'a>>,

    /// The visibility of the struct
    pub visibility: Option<Visibility<'a>>,

    /// The name of the struct
    pub name: Cow<'a, Identifier>,

    /// The name of the modifiable version of the struct
    pub modifiable_name: Identifier,

    /// The generic parameters of the struct, if any
    pub generic_params: Option<GenericParams<'a>>,

    /// The where clause of the struct, if any
    pub where_clause: Option<WhereClause<'a>>,

    /// The fields of the struct
    pub fields: Vec<SettingsCacheInputField<'a>>,
}
