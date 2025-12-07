use crate::{
    graphics::{MeshRenderer, MeshRenderers},
    util::{ArenaIter, ArenaIterMut},
};

impl MeshRenderers {
    /// Get an iterator over all the registered [`MeshRenderer`]s
    pub fn iter<'a>(&'a self) -> ArenaIter<'a, MeshRenderer> {
        self.arena.iter()
    }

    /// Get an iterator over all the registered [`MeshRenderer`]s, returning mutable references
    pub fn iter_mut<'a>(&'a mut self) -> ArenaIterMut<'a, MeshRenderer> {
        self.arena.iter_mut()
    }
}

impl<'a> IntoIterator for &'a MeshRenderers {
    type Item = &'a MeshRenderer;
    type IntoIter = ArenaIter<'a, MeshRenderer>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut MeshRenderers {
    type Item = &'a mut MeshRenderer;
    type IntoIter = ArenaIterMut<'a, MeshRenderer>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}
