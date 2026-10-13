use crate::update::ecs::{Archetype, ArchetypeSet};

impl ArchetypeSet {
    /// Get a reference to the archetype at `index`
    pub fn get(&self, index: usize) -> Option<&Archetype> {
        self.archetypes.get(index)
    }

    /// Get a mutable reference to the archetype at `index`
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Archetype> {
        self.archetypes.get_mut(index)
    }

    /// Get two mutable references to the archetypes at `index1` and `index2`
    pub fn get_disjoint_mut(&mut self, index1: usize, index2: usize) -> [&mut Archetype; 2] {
        self.archetypes
            .get_disjoint_mut([index1, index2])
            .expect("cannot get disjoint mutable references to archetypes at the same index")
    }
}
