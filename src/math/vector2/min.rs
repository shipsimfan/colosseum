use crate::math::{number::Min, Vector2};

impl<T: Min> Vector2<T> {
    /// Compares and returns the minimum of two vectors component-wise
    pub fn min_v(self, other: Vector2<T>) -> Vector2<T> {
        Vector2::new(self.x.min(other.x), self.y.min(other.y))
    }
}

impl<T: Min + Clone> Vector2<T> {
    /// Compares and returns the minimum of a vector component-wise and a scalar
    pub fn min(self, other: T) -> Vector2<T> {
        Vector2::new(self.x.min(other.clone()), self.y.min(other))
    }
}

impl<T: Min> Min for Vector2<T> {
    fn min(self, other: Self) -> Self {
        self.min_v(other)
    }
}
