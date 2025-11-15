use crate::settings_cache::output::{
    SettingsCacheGetFunction, SettingsCacheLoad, SettingsCacheOutput, SettingsCacheSave,
    SettingsCacheSetFunction, SettingsCacheStructCreation,
};
use proc_macro_util::{
    Result, Span, Token,
    ast::{
        Item, ItemKind, Type, TypePath, TypePathSegment, VisItemKind,
        items::{StructBody, StructField, StructFields},
    },
    tokens::Identifier,
};

/// Add a new field to `fields`
fn add_field<'a, 'b>(
    fields: &'b mut Option<StructFields<'a>>,
    name: &str,
    r#type: &[&str],
) -> &'b mut StructFields<'a> {
    assert!(r#type.len() > 0);
    let mut type_path = TypePath {
        leading: Some(Token![::]()),
        first: TypePathSegment::from_ident(Identifier::new(r#type[0])),
        remaining: Vec::new(),
    };
    for r#type in &r#type[1..] {
        type_path.remaining.push((
            Token![::](),
            TypePathSegment::from_ident(Identifier::new(r#type)),
        ));
    }
    let r#type = Type::Path(type_path);

    let field = StructField {
        attributes: Vec::new(),
        visibility: None,
        name: Identifier::new(name).into(),
        colon: Token![:](),
        r#type,
    };

    match fields {
        None => {
            *fields = Some(StructFields {
                first: field,
                remaining: Vec::new(),
                last: None,
            })
        }
        Some(fields) => fields.remaining.push((Token![,](), field)),
    }

    fields.as_mut().unwrap()
}

fn foreach_field<'a, F: FnMut(&StructField<'a>)>(fields: &StructFields<'a>, mut f: F) {
    f(&fields.first);

    for (_, field) in &fields.remaining {
        f(field);
    }
}

impl<'a> SettingsCacheOutput<'a> {
    /// Create a new [`SettingsCacheOutput`] from `r#struct`
    pub fn from_input(mut item: Item<'a>) -> Result<SettingsCacheOutput<'a>> {
        // Verify we got a struct
        let r#struct = match &mut item.kind {
            ItemKind::Vis(vis_item) => match &mut vis_item.kind {
                VisItemKind::Struct(r#struct) => r#struct,
                _ => return Err(Span::call_site().error("must be applied to a struct")),
            },
            _ => return Err(Span::call_site().error("must be applied to a struct")),
        };
        let name = r#struct.name.clone();

        // Validate struct body type
        let fields = match &mut r#struct.body {
            StructBody::Normal {
                where_clause: _,
                fields,
            } => fields,
            _ => return Err(Span::call_site().error("struct must contain normal fields")),
        };

        // Add additional fields to struct
        let fields = add_field(
            fields,
            "graphics_settings",
            &["colosseum", "graphics", "GraphicsSettings"],
        );

        // Collect elements for fields
        let field_count = fields.remaining.len() + 1;
        let mut get_functions = Vec::with_capacity(field_count);
        let mut set_functions = Vec::with_capacity(field_count);
        let mut loads = Vec::with_capacity(field_count);
        let mut saves = Vec::with_capacity(field_count);
        let mut struct_creation = Vec::with_capacity(field_count);
        foreach_field(fields, |field| {
            get_functions.push(SettingsCacheGetFunction::from_input(field));
            set_functions.push(SettingsCacheSetFunction::from_input(field));
            loads.push(SettingsCacheLoad::from_input(field));
            saves.push(SettingsCacheSave::from_input(field));
            struct_creation.push(SettingsCacheStructCreation::from_input(field));
        });

        // Create output
        Ok(SettingsCacheOutput {
            item,
            name,
            get_functions,
            set_functions,
            loads,
            saves,
            struct_creation,
        })
    }
}
