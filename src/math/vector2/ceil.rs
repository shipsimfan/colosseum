use crate::math::{number::Ceil, Vector2};

impl<T: Ceil> Vector2<T> {
    /// Rounds the values of [`Vector2`] component wise up to the nearest integer
    pub fn ceil(self) -> Self {
        Vector2::new(self.x.ceil(), self.y.ceil())
    }
}

impl<T: Ceil> Ceil for Vector2<T> {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
