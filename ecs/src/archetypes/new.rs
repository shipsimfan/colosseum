use crate::{
    Archetypes, Entity,
    archetypes::{Archetype, Components},
};
use alexandria::{Id, SlotMap};

impl Archetypes {
    /// Create a new, empty set of [`Archetypes`]
    pub fn new() -> Archetypes {
        let entity_component = Components::new::<Id<Entity>>();
        let entity_archetype = Archetype::new(vec![entity_component]);

        Archetypes {
            entities: SlotMap::new(),
            archetypes: vec![entity_archetype],
        }
    }
}
