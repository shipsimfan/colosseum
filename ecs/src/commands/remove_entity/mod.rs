use crate::{Commands, Entity};
use alexandria::Id;

mod command;
mod new;

/// Remove an entity from the ECS
pub struct RemoveEntity {
    /// The entity to remove
    entity: Id<Entity>,
}

impl Commands {
    /// Remove an entity from the ECS
    pub fn remove_entity(&mut self, entity: Id<Entity>) {
        self.queue(RemoveEntity::new(entity));
    }
}
