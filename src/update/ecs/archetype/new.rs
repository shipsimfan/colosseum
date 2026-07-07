use crate::update::ecs::{Archetype, archetype::Components};

impl Archetype {
    /// Create a new [`Archetype`]
    pub(in crate::update::ecs::archetype) fn new(components: Vec<Components>) -> Archetype {
        let mut component_ids: Box<[_]> = components.iter().map(Components::type_id).collect();
        component_ids.sort_unstable();

        Archetype {
            component_ids,
            components: components.into_boxed_slice(),
            next_archetypes: Vec::new(),
            prev_archetypes: Vec::new(),
        }
    }
}
