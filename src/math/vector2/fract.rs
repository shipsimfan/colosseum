use crate::math::{number::Fract, Vector2};

impl<T: Fract> Vector2<T> {
    /// Gets the fractional parts of the values of a [`Vector2`] component-wise
    pub fn fract(self) -> Self {
        Vector2::new(self.x.fract(), self.y.fract())
    }
}

impl<T: Fract> Fract for Vector2<T> {
    fn fract(self) -> Self {
        self.fract()
    }
}
