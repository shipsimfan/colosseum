use crate::QueryFilter;
use std::{any::TypeId, marker::PhantomData};

/// A filter that matches if an entity does not have a specific component
pub struct Without<Component> {
    /// A marker for the component
    _marker: PhantomData<fn() -> Component>,
}

impl<Component: 'static> QueryFilter for Without<Component> {
    fn filter(components: &[TypeId]) -> bool {
        !components.contains(&TypeId::of::<Component>())
    }
}
