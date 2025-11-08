use crate::math::{number::Max, Vector2};

impl<T: Max> Vector2<T> {
    /// Compares and returns the maximum of two vectors component-wise
    pub fn max_v(self, other: Vector2<T>) -> Vector2<T> {
        Vector2::new(self.x.max(other.x), self.y.max(other.y))
    }
}

impl<T: Max + Clone> Vector2<T> {
    /// Compares and returns the maximum of a vector component-wise and a scalar
    pub fn max(self, other: T) -> Vector2<T> {
        Vector2::new(self.x.max(other.clone()), self.y.max(other))
    }
}

impl<T: Max> Max for Vector2<T> {
    fn max(self, other: Self) -> Self {
        self.max_v(other)
    }
}
