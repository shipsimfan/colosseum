use crate::{
    graphics::{PointLight, PointLights},
    util::{ArenaIter, ArenaIterMut},
};

impl PointLights {
    /// Get an iterator over all the registered [`PointLight`]s
    pub fn iter<'a>(&'a self) -> ArenaIter<'a, PointLight> {
        self.list.iter()
    }

    /// Get an iterator over all the registered [`PointLight`]s, returning mutable references
    pub fn iter_mut<'a>(&'a mut self) -> ArenaIterMut<'a, PointLight> {
        self.list.iter_mut()
    }
}

impl<'a> IntoIterator for &'a PointLights {
    type Item = &'a PointLight;
    type IntoIter = ArenaIter<'a, PointLight>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut PointLights {
    type Item = &'a mut PointLight;
    type IntoIter = ArenaIterMut<'a, PointLight>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}
