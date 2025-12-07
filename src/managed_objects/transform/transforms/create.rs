use crate::{Transform, TransformHandle, Transforms};

impl Transforms {
    /// Create a new [`Transform`]
    pub fn create(&mut self) -> TransformHandle {
        self.arena.insert(Transform::new())
    }
}
