use crate::{
    Archetypes,
    archetypes::{Archetype, Components},
};
use std::any::TypeId;

impl Archetypes {
    /// Push a new archetype to the set of archetypes
    pub(in crate::archetypes) fn push<T: 'static + Send + Sync + Sized>(
        &mut self,
        archetype_index: usize,
    ) -> usize {
        // Create the new archetype
        let mut new_archetype = self.archetypes[archetype_index].extend(Components::new::<T>());
        let new_archetype_index = self.archetypes.len();

        // Check for archetypes that are one away from the new archetype and update their next_archetype mapping
        for (i, archetype) in self.archetypes.iter_mut().enumerate() {
            if let Some(type_id) = only_one_addition(archetype, &new_archetype) {
                archetype.push_next_archetype(type_id, new_archetype_index);
                new_archetype.push_prev_archetype(type_id, i);
            } else if let Some(type_id) = only_one_addition(&new_archetype, archetype) {
                archetype.push_prev_archetype(type_id, new_archetype_index);
                new_archetype.push_next_archetype(type_id, i);
            }
        }

        // Insert the new archetype into the set of archetypes
        self.archetypes.push(new_archetype);
        new_archetype_index
    }
}

/// Check if archetype `a` only needs one additional component to become archetype `b`
fn only_one_addition(a: &Archetype, b: &Archetype) -> Option<TypeId> {
    let a = a.component_ids();
    let b = b.component_ids();

    if a.len() + 1 != b.len() {
        return None;
    }

    let mut i = 0;
    let mut missing = None;
    for &b in b {
        if i < a.len() {
            if a[i] == b {
                i += 1;
            } else if a[i] < b {
                return None;
            } else {
                if missing.is_some() {
                    return None;
                }
                missing = Some(b);
            }
        } else {
            if missing.is_some() {
                return None;
            }
            missing = Some(b);
        }
    }

    if i == a.len() { missing } else { None }
}
