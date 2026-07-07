use crate::update::ecs::{Archetype, ArchetypeSet};
use std::ops::{Index, IndexMut};

impl Index<usize> for ArchetypeSet {
    type Output = Archetype;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).expect("`Archetype` index out of bounds")
    }
}

impl IndexMut<usize> for ArchetypeSet {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index)
            .expect("`Archetype` index out of bounds")
    }
}
