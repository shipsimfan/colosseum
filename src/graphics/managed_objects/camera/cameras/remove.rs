use crate::graphics::{CameraHandle, Cameras};

impl Cameras {
    /// Remove the [`Camera`](crate::graphics::Camera) identified by `handle`
    pub fn remove(&mut self, handle: CameraHandle) {
        self.arena.remove(handle);
    }
}
