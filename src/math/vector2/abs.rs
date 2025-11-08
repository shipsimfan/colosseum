use crate::math::{number::Absolute, Vector2};

impl<T: Absolute> Vector2<T> {
    /// Gets the aboslute value of a [`Vector2`] component wise
    pub fn abs(self) -> Self {
        Vector2::new(self.x.abs(), self.y.abs())
    }
}

impl<T: Absolute> Absolute for Vector2<T> {
    fn abs(self) -> Self {
        self.abs()
    }
}
