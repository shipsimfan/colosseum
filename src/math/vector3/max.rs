use crate::math::{number::Max, Vector3};

impl<T: Max> Vector3<T> {
    /// Compares and returns the maximum of two vectors component-wise
    pub fn max_v(self, other: Vector3<T>) -> Vector3<T> {
        Vector3::new(
            self.x.max(other.x),
            self.y.max(other.y),
            self.z.max(other.z),
        )
    }
}

impl<T: Max + Clone> Vector3<T> {
    /// Compares and returns the maximum of a vector component-wise and a scalar
    pub fn max(self, other: T) -> Vector3<T> {
        Vector3::new(
            self.x.max(other.clone()),
            self.y.max(other.clone()),
            self.z.max(other),
        )
    }
}

impl<T: Max> Max for Vector3<T> {
    fn max(self, other: Self) -> Self {
        self.max_v(other)
    }
}
