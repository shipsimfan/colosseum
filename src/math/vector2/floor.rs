use crate::math::{number::Floor, Vector2};

impl<T: Floor> Vector2<T> {
    /// Rounds the values of [`Vector2`] component wise down to the nearest integer
    pub fn floor(self) -> Self {
        Vector2::new(self.x.floor(), self.y.floor())
    }
}

impl<T: Floor> Floor for Vector2<T> {
    fn floor(self) -> Self {
        self.floor()
    }
}
