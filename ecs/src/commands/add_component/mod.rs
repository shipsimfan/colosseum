use crate::{Commands, Entity};
use alexandria::Id;

mod command;
mod new;

/// Add a component to an entity
pub struct AddComponent<Component: 'static + Send + Sync + Sized> {
    /// The entity to which the component will be added
    entity: Id<Entity>,

    /// The component to be added to the entity
    component: Component,
}

impl Commands {
    /// Add a component to an entity
    pub fn add_component<Component: 'static + Send + Sync + Sized>(
        &mut self,
        entity: Id<Entity>,
        component: Component,
    ) {
        self.queue(AddComponent::new(entity, component));
    }
}
