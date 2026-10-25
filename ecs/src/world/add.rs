use crate::{ComponentList, Entity, World};
use alexandria::Id;

impl World {
    /// Add an entity with the specified components to the world
    pub fn add<Components: ComponentList>(&mut self, components: Components) -> Id<Entity> {
        let entity = self.create_entity();
        components.add_to_entity(entity, self);
        entity
    }

    /// Add a component to an entity
    pub fn add_component<T: 'static + Send + Sync + Sized>(
        &mut self,
        entity: Id<Entity>,
        component: T,
    ) {
        if let Some((new_archetype_index, new_archetype_types)) =
            self.archetypes.add_component(entity, component)
        {
            // TODO: Register the new archetype with systems
        }
    }
}
