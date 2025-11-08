use crate::math::{number::Min, Vector4};

impl<T: Min> Vector4<T> {
    /// Compares and returns the minimum of two vectors component-wise
    pub fn min_v(self, other: Vector4<T>) -> Vector4<T> {
        Vector4::new(
            self.x.min(other.x),
            self.y.min(other.y),
            self.z.min(other.z),
            self.w.min(other.w),
        )
    }
}

impl<T: Min + Clone> Vector4<T> {
    /// Compares and returns the minimum of a vector component-wise and a scalar
    pub fn min(self, other: T) -> Vector4<T> {
        Vector4::new(
            self.x.min(other.clone()),
            self.y.min(other.clone()),
            self.z.min(other.clone()),
            self.w.min(other),
        )
    }
}

impl<T: Min> Min for Vector4<T> {
    fn min(self, other: Self) -> Self {
        self.min_v(other)
    }
}
