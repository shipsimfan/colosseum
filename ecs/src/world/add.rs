use crate::{ComponentList, Entity, World};
use alexandria::Id;

impl World {
    /// Add a component to an entity
    pub fn add_component<Component: 'static + Send + Sync + Sized>(
        &mut self,
        entity: Id<Entity>,
        component: Component,
    ) {
        if let Some((new_archetype_index, new_archetype_types)) =
            self.archetypes.add_component(entity, component)
        {
            self.systems
                .register_archetype(new_archetype_index, &new_archetype_types);
        }
    }

    /// Add multiple components to an entity
    pub fn add_components<Components: ComponentList>(
        &mut self,
        entity: Id<Entity>,
        components: Components,
    ) {
        components.add_to_entity(entity, self);
    }
}
