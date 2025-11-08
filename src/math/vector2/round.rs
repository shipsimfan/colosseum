use crate::math::{number::Round, Vector2};

impl<T: Round> Vector2<T> {
    /// Rounds the values of [`Vector2`] component wise to the nearest integer
    pub fn round(self) -> Self {
        Vector2::new(self.x.round(), self.y.round())
    }
}

impl<T: Round> Round for Vector2<T> {
    fn round(self) -> Self {
        self.round()
    }
}
