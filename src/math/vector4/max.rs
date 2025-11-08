use crate::math::{number::Max, Vector4};

impl<T: Max> Vector4<T> {
    /// Compares and returns the maximum of two vectors component-wise
    pub fn max_v(self, other: Vector4<T>) -> Vector4<T> {
        Vector4::new(
            self.x.max(other.x),
            self.y.max(other.y),
            self.z.max(other.z),
            self.w.max(other.w),
        )
    }
}

impl<T: Max + Clone> Vector4<T> {
    /// Compares and returns the maximum of a vector component-wise and a scalar
    pub fn max(self, other: T) -> Vector4<T> {
        Vector4::new(
            self.x.max(other.clone()),
            self.y.max(other.clone()),
            self.z.max(other.clone()),
            self.w.max(other),
        )
    }
}

impl<T: Max> Max for Vector4<T> {
    fn max(self, other: Self) -> Self {
        self.max_v(other)
    }
}
