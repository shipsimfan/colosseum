use crate::{Commands, Entity};
use alexandria::Id;
use std::marker::PhantomData;

mod command;
mod new;

/// Remove a component from an entity
pub struct RemoveComponent<Component: 'static> {
    /// The entity from which the component will be removed
    entity: Id<Entity>,

    /// A marker for the component type to be removed
    _marker: PhantomData<Component>,
}

impl Commands {
    /// Remove a component from an entity
    pub fn remove_component<Component: 'static>(&mut self, entity: Id<Entity>) {
        self.queue(RemoveComponent::<Component>::new(entity));
    }
}
