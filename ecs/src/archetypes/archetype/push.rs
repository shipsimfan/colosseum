use crate::archetypes::Archetype;
use std::any::TypeId;

impl Archetype {
    /// Push a new entity into the archetype with the specified component data
    pub fn push<'a, I: IntoIterator<Item = (&'a [u8], TypeId)>>(&mut self, component_data: I) {
        let mut i = 0;
        for (data, type_id) in component_data {
            let index = self
                .components
                .iter()
                .position(|component| component.type_id() == type_id)
                .expect("component type not found in archetype");

            self.components[index].push(data);

            i += 1;
        }

        debug_assert_eq!(
            i,
            self.component_ids.len(),
            "not all components were provided for the new entity"
        );
    }

    /// Push a new next archetype for this archetype
    pub fn push_next_archetype(&mut self, type_id: TypeId, archetype_index: usize) {
        self.next_archetypes.push((type_id, archetype_index));
    }

    /// Push a new previous archetype for this archetype
    pub fn push_prev_archetype(&mut self, type_id: TypeId, archetype_index: usize) {
        self.prev_archetypes.push((type_id, archetype_index));
    }
}
