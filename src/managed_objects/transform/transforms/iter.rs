use crate::{
    Transform, Transforms,
    util::{ArenaIter, ArenaIterMut},
};

impl Transforms {
    /// Get an iterator over all the registered [`Transform`]s
    pub fn iter<'a>(&'a self) -> ArenaIter<'a, Transform> {
        self.arena.iter()
    }

    /// Get an iterator over all the registered [`Transform`]s, returning mutable references
    pub fn iter_mut<'a>(&'a mut self) -> ArenaIterMut<'a, Transform> {
        self.arena.iter_mut()
    }
}

impl<'a> IntoIterator for &'a Transforms {
    type Item = &'a Transform;
    type IntoIter = ArenaIter<'a, Transform>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut Transforms {
    type Item = &'a mut Transform;
    type IntoIter = ArenaIterMut<'a, Transform>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}
