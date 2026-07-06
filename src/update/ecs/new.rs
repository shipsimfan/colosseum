use crate::update::{ECS, ecs::ArchetypeSet};
use alexandria::SlotMap;

impl ECS {
    /// Create a new [`ECS`] system
    pub fn new() -> ECS {
        ECS {
            entities: SlotMap::new(),
            archetypes: ArchetypeSet::new(),
        }
    }
}
