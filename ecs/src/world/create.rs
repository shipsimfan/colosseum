use crate::{Entity, World};
use alexandria::Id;

impl World {
    /// Create a new entity in the world
    pub fn create_entity(&mut self) -> Id<Entity> {
        self.archetypes.create_entity()
    }
}
