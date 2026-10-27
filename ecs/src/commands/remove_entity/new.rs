use crate::{Entity, RemoveEntity};
use alexandria::Id;

impl RemoveEntity {
    /// Create a new [`RemoveEntity`] command
    pub fn new(entity: Id<Entity>) -> RemoveEntity {
        RemoveEntity { entity }
    }
}
