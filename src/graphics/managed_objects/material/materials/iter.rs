use crate::{
    graphics::{Material, Materials},
    util::{ArenaIter, ArenaIterMut},
};

impl Materials {
    /// Get an iterator over all the registered [`Material`]s
    pub fn iter<'a>(&'a self) -> ArenaIter<'a, Material> {
        self.arena.iter()
    }

    /// Get an iterator over all the registered [`Material`]s, returning mutable references
    pub fn iter_mut<'a>(&'a mut self) -> ArenaIterMut<'a, Material> {
        self.arena.iter_mut()
    }
}

impl<'a> IntoIterator for &'a Materials {
    type Item = &'a Material;
    type IntoIter = ArenaIter<'a, Material>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut Materials {
    type Item = &'a mut Material;
    type IntoIter = ArenaIterMut<'a, Material>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}
