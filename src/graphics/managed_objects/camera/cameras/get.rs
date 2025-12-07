use crate::graphics::{Camera, CameraHandle, Cameras};

impl Cameras {
    /// Get the [`Camera`] at `handle`
    pub fn get(&self, handle: CameraHandle) -> Option<&Camera> {
        self.arena.get(handle)
    }

    /// Get the [`Camera`] at `handle` mutably
    pub fn get_mut(&mut self, handle: CameraHandle) -> Option<&mut Camera> {
        self.arena.get_mut(handle)
    }
}
