use crate::{TransformHandle, Transforms};

impl Transforms {
    /// Remove the [`Transform`](crate::Transform) identified by `handle`
    pub fn remove(&mut self, handle: TransformHandle) {
        self.arena.remove(handle);
    }
}
