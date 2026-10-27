use crate::{AddComponents, ComponentList, Entity};
use alexandria::Id;

impl<Components: ComponentList> AddComponents<Components> {
    /// Create a new [`AddComponents`] command
    pub fn new(entity: Id<Entity>, components: Components) -> AddComponents<Components> {
        AddComponents { entity, components }
    }
}
