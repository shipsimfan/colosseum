use crate::{Transform, TransformHandle, Transforms};

impl Transforms {
    /// Get the [`Transform`] at `handle`
    pub fn get(&self, handle: TransformHandle) -> Option<&Transform> {
        self.arena.get(handle)
    }

    /// Get the [`Transform`] at `handle` mutably
    pub fn get_mut(&mut self, handle: TransformHandle) -> Option<&mut Transform> {
        self.arena.get_mut(handle)
    }
}
