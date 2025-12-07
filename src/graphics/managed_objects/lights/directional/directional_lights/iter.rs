use crate::{
    graphics::{DirectionalLight, DirectionalLights},
    util::{ArenaIter, ArenaIterMut},
};

impl DirectionalLights {
    /// Get an iterator over all the registered [`DirectionalLight`]s
    pub fn iter<'a>(&'a self) -> ArenaIter<'a, DirectionalLight> {
        self.list.iter()
    }

    /// Get an iterator over all the registered [`DirectionalLight`]s, returning mutable references
    pub fn iter_mut<'a>(&'a mut self) -> ArenaIterMut<'a, DirectionalLight> {
        self.list.iter_mut()
    }
}

impl<'a> IntoIterator for &'a DirectionalLights {
    type Item = &'a DirectionalLight;
    type IntoIter = ArenaIter<'a, DirectionalLight>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut DirectionalLights {
    type Item = &'a mut DirectionalLight;
    type IntoIter = ArenaIterMut<'a, DirectionalLight>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}
