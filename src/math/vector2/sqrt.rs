use crate::math::{number::Sqrt, Vector2};

impl<T: Sqrt> Vector2<T> {
    /// Gets the component-wise square root
    pub fn sqrt(self) -> Self {
        Vector2::new(self.x.sqrt(), self.y.sqrt())
    }
}

impl<T: Sqrt> Sqrt for Vector2<T> {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
