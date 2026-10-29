use crate::QueryFilter;
use std::{any::TypeId, marker::PhantomData};

/// A filter that matches if an entity has a specific component
pub struct With<Component> {
    /// A marker for the component
    _marker: PhantomData<fn() -> Component>,
}

impl<Component: 'static> QueryFilter for With<Component> {
    fn filter(components: &[TypeId]) -> bool {
        components.contains(&TypeId::of::<Component>())
    }
}
