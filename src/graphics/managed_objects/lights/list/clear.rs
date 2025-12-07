use crate::graphics::managed_objects::lights::{LightList, LightType};

impl<T: LightType> LightList<T> {
    /// Remove all registered lights from this list
    pub(crate) fn clear(&mut self) {
        self.arena.clear();
    }
}
