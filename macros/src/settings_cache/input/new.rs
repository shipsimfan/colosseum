use crate::settings_cache::SettingsCacheInput;
use proc_macro_util::{
    Result, Span,
    ast::{Item, ItemKind, VisItemKind, Visibility, items::Struct},
};

impl<'a> SettingsCacheInput<'a> {
    /// Parse the input to the settings cache macro
    pub fn new(item: Item<'a>) -> Result<SettingsCacheInput<'a>> {
        // Extract the attributes from the item
        let attributes = item.attributes;

        // Validate the item is a struct
        let (visibility, r#struct) = extract_struct(item.kind)
            .ok_or_else(|| Span::call_site().error("must be applied to a struct"))?;

        Ok(SettingsCacheInput {
            attributes,
            visibility,
            name: r#struct.name,
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
