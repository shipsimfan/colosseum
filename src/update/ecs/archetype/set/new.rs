use crate::{
    logging::Logger,
    update::{
        Entity,
        ecs::{Archetype, ArchetypeSet, archetype::Components},
    },
};
use alexandria::{Id, PackedMap, SlotMap};

impl ArchetypeSet {
    /// Create a new [`ArchetypeSet`]
    pub fn new(logger: Logger) -> ArchetypeSet {
        let entity_component = Components::new::<Id<Entity>>();
        let entity_archetype = Archetype::new(vec![entity_component], &logger);

        let rendering_systems = vec![];

        ArchetypeSet {
            archetypes: vec![entity_archetype],
            pre_update_systems: PackedMap::new(),
            ad_hoc_systems: SlotMap::new(),
            post_update_systems: PackedMap::new(),
            rendering_systems,
            logger,
        }
    }
}
