use crate::graphics::{MaterialHandle, Materials};

impl Materials {
    /// Remove the [`Material`](crate::graphics::Material) identified by `handle`
    pub fn remove(&mut self, handle: MaterialHandle) {
        self.arena.remove(handle);
    }
}
