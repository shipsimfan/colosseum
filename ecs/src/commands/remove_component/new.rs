use crate::{Entity, RemoveComponent};
use alexandria::Id;
use std::marker::PhantomData;

impl<Component: 'static> RemoveComponent<Component> {
    /// Create a new [`RemoveComponent`] command
    pub fn new(entity: Id<Entity>) -> RemoveComponent<Component> {
        RemoveComponent {
            entity,
            _marker: PhantomData,
        }
    }
}
