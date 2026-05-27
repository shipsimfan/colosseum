use crate::settings_cache::SettingsCacheInputField;
use proc_macro_util::{
    Result, Token,
    ast::{
        PathIdentSegment, Type, TypePath, TypePathSegment,
        items::{StructField, StructFields},
    },
    tokens::Identifier,
};

const BUILT_IN_GROUPS: &[(&str, &str)] = &[("display", "DisplaySettings")];

impl<'a> SettingsCacheInputField<'a> {
    /// Create a new set of [`SettingsCacheInputField`]s from the [`StructFields`]
    pub fn new_set(fields: Option<StructFields<'a>>) -> Result<Vec<SettingsCacheInputField<'a>>> {
        let mut set = SettingsCacheInputField::parse_set(fields)?;

        set.reserve(BUILT_IN_GROUPS.len());
        for (group_name, group_type) in BUILT_IN_GROUPS {
            set.push(SettingsCacheInputField {
                attributes: Vec::new(),
                name: Identifier::new(group_name).into(),
                r#type: built_type_name(&["colosseum", "settings", group_type]),
            });
        }

        Ok(set)
    }

    pub fn parse_set(fields: Option<StructFields<'a>>) -> Result<Vec<SettingsCacheInputField<'a>>> {
        let fields = match fields {
            Some(fields) => fields,
            None => return Ok(Vec::new()),
        };

        let mut result = Vec::with_capacity(fields.remaining.len() + 1);

        result.push(SettingsCacheInputField::new(fields.first)?);
        for (_, field) in fields.remaining {
            result.push(SettingsCacheInputField::new(field)?);
        }

        Ok(result)
    }

    /// Create a new [`SettingsCacheInputField`] from a [`StructField`]
    pub fn new(field: StructField<'a>) -> Result<SettingsCacheInputField<'a>> {
        Ok(SettingsCacheInputField {
            attributes: field.attributes,
            name: field.name,
            r#type: field.r#type,
        })
    }
}

/// Build a [`Type`] from a built-in group name
fn built_type_name(name: &[&str]) -> Type<'static> {
    Type::Path(TypePath {
        leading: Some(Token![::]()),
        first: build_type_path_segment(name[0]),
        remaining: name[1..]
            .iter()
            .map(|segment| (Token![::](), build_type_path_segment(segment)))
            .collect(),
    })
}

fn build_type_path_segment(name: &str) -> TypePathSegment<'static> {
    TypePathSegment {
        ident: PathIdentSegment::Identifier(Identifier::new(name).into()),
        generics: None,
    }
}
