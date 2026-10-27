use crate::{ComponentList, Entity, World};
use alexandria::Id;

impl World {
    /// Spawn a new entity with the specified components to the world
    pub fn spawn<Components: ComponentList>(&mut self, components: Components) -> Id<Entity> {
        let entity = self.create_entity();
        components.add_to_entity(entity, self);
        entity
    }
}
