use crate::math::{number::Min, Vector3};

impl<T: Min> Vector3<T> {
    /// Compares and returns the minimum of two vectors component-wise
    pub fn min_v(self, other: Vector3<T>) -> Vector3<T> {
        Vector3::new(
            self.x.min(other.x),
            self.y.min(other.y),
            self.z.min(other.z),
        )
    }
}

impl<T: Min + Clone> Vector3<T> {
    /// Compares and returns the minimum of a vector component-wise and a scalar
    pub fn min(self, other: T) -> Vector3<T> {
        Vector3::new(
            self.x.min(other.clone()),
            self.y.min(other.clone()),
            self.z.min(other),
        )
    }
}

impl<T: Min> Min for Vector3<T> {
    fn min(self, other: Self) -> Self {
        self.min_v(other)
    }
}
