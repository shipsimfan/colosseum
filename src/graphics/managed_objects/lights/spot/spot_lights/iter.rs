use crate::{
    graphics::{SpotLight, SpotLights},
    util::{ArenaIter, ArenaIterMut},
};

impl SpotLights {
    /// Get an iterator over all the registered [`SpotLight`]s
    pub fn iter<'a>(&'a self) -> ArenaIter<'a, SpotLight> {
        self.list.iter()
    }

    /// Get an iterator over all the registered [`SpotLight`]s, returning mutable references
    pub fn iter_mut<'a>(&'a mut self) -> ArenaIterMut<'a, SpotLight> {
        self.list.iter_mut()
    }
}

impl<'a> IntoIterator for &'a SpotLights {
    type Item = &'a SpotLight;
    type IntoIter = ArenaIter<'a, SpotLight>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut SpotLights {
    type Item = &'a mut SpotLight;
    type IntoIter = ArenaIterMut<'a, SpotLight>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}
