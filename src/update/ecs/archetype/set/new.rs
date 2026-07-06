use crate::update::{
    Entity,
    ecs::{Archetype, ArchetypeSet, archetype::Components},
};
use alexandria::Id;

impl ArchetypeSet {
    /// Create a new [`ArchetypeSet`]
    pub fn new() -> ArchetypeSet {
        let entity_component = Components::new::<Id<Entity>>();
        let entity_archetype = Archetype::new_one(entity_component);

        ArchetypeSet {
            archetypes: vec![entity_archetype],
        }
    }
}
