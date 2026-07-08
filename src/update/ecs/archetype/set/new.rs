use crate::{
    logging::Logger,
    update::{
        Entity,
        ecs::{Archetype, ArchetypeSet, archetype::Components},
    },
};
use alexandria::{Id, PackedMap};

impl ArchetypeSet {
    /// Create a new [`ArchetypeSet`]
    pub fn new(logger: Logger) -> ArchetypeSet {
        let entity_component = Components::new::<Id<Entity>>();
        let entity_archetype = Archetype::new(vec![entity_component], &logger);

        ArchetypeSet {
            archetypes: vec![entity_archetype],
            ad_hoc_systems: PackedMap::new(),
            logger,
        }
    }
}
