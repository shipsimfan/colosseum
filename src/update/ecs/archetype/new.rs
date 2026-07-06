use crate::update::ecs::{Archetype, archetype::Components};

impl Archetype {
    /// Create a new [`Archetype`]
    pub(in crate::update::ecs::archetype) fn new_one(components: Components) -> Archetype {
        let component_ids = vec![components.type_id()].into_boxed_slice();

        Archetype {
            component_ids,
            components: vec![components].into_boxed_slice(),
        }
    }
}
