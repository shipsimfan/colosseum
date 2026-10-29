use crate::archetypes::{Archetype, Components};

impl Archetype {
    /// Create a new [`Archetype`]
    pub fn new(components: Vec<Components>) -> Archetype {
        let component_ids: Box<[_]> = components.iter().map(Components::type_id).collect();

        let mut sorted_component_ids = component_ids.clone();
        sorted_component_ids.sort();

        Archetype {
            sorted_component_ids,
            component_ids,
            components: components.into_boxed_slice(),
            next_archetypes: Vec::new(),
            prev_archetypes: Vec::new(),
        }
    }
}
