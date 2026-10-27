use crate::{Commands, ComponentList, Entity};
use alexandria::Id;

mod command;
mod new;

/// Add multiple components to an entity
pub struct AddComponents<Components: ComponentList> {
    /// The entity to which the components will be added
    entity: Id<Entity>,

    /// The components to be added to the entity
    components: Components,
}

impl Commands {
    /// Add multiple components to an entity
    pub fn add_components<Components: ComponentList>(
        &mut self,
        entity: Id<Entity>,
        components: Components,
    ) {
        self.queue(AddComponents::new(entity, components));
    }
}
