use crate::settings_cache::{SettingsCacheInput, SettingsCacheInputField};
use proc_macro_util::{
    Result, Span,
    ast::{
        Item, ItemKind, VisItemKind, Visibility, WhereClause,
        items::{Struct, StructBody, StructFields},
    },
    tokens::Identifier,
};

impl<'a> SettingsCacheInput<'a> {
    /// Parse the input to the settings cache macro
    pub fn new(item: Item<'a>) -> Result<SettingsCacheInput<'a>> {
        // Extract the attributes from the item
        let attributes = item.attributes;

        // Validate the item is a struct
        let (visibility, r#struct) = extract_struct(item.kind)
            .ok_or_else(|| Span::call_site().error("must be applied to a struct"))?;

        let modifiable_name = Identifier::new(&format!("Modifiable{}", r#struct.name));

        // Extract the struct fields
        let (where_clause, struct_fields) = extract_fields(r#struct.body)?;

        Ok(SettingsCacheInput {
            attributes,
            visibility,
            name: r#struct.name,
            modifiable_name,
            generic_params: r#struct.generic_params,
            where_clause,
            fields: SettingsCacheInputField::new_set(struct_fields)?,
        })
    }
}

/// Extract a [`Struct`] from an [`ItemKind`]
fn extract_struct<'a>(item: ItemKind<'a>) -> Option<(Option<Visibility<'a>>, Struct<'a>)> {
    let vis_item = match item {
        ItemKind::Vis(vis_item) => vis_item,
        _ => return None,
    };

    let visibility = vis_item.visibility;
    let r#struct = match vis_item.kind {
        VisItemKind::Struct(r#struct) => r#struct,
        _ => return None,
    };

    Some((visibility, r#struct))
}

/// Extract a [`StructFields`] from a [`StructBody`]
fn extract_fields<'a>(
    body: StructBody<'a>,
) -> Result<(Option<WhereClause<'a>>, Option<StructFields<'a>>)> {
    match body {
        StructBody::Normal {
            where_clause,
            fields,
        } => Ok((where_clause, fields)),
        _ => return Err(Span::call_site().error("only normal structs are supported")),
    }
}
