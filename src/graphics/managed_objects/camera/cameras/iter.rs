use crate::{
    graphics::{Camera, Cameras},
    util::{ArenaIter, ArenaIterMut},
};

impl Cameras {
    /// Get an iterator over all the registered [`Camera`]s
    pub fn iter<'a>(&'a self) -> ArenaIter<'a, Camera> {
        self.arena.iter()
    }

    /// Get an iterator over all the registered [`Camera`]s, returning mutable references
    pub fn iter_mut<'a>(&'a mut self) -> ArenaIterMut<'a, Camera> {
        self.arena.iter_mut()
    }
}

impl<'a> IntoIterator for &'a Cameras {
    type Item = &'a Camera;
    type IntoIter = ArenaIter<'a, Camera>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut Cameras {
    type Item = &'a mut Camera;
    type IntoIter = ArenaIterMut<'a, Camera>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}
