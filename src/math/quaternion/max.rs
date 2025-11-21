use crate::math::{number::Max, Quaternion};

impl<T: Max> Quaternion<T> {
    /// Compares and returns the maximum of two vectors component-wise
    pub fn max_v(self, other: Quaternion<T>) -> Quaternion<T> {
        Quaternion::new(
            self.x.max(other.x),
            self.y.max(other.y),
            self.z.max(other.z),
            self.w.max(other.w),
        )
    }
}

impl<T: Max + Clone> Quaternion<T> {
    /// Compares and returns the maximum of a vector component-wise and a scalar
    pub fn max(self, other: T) -> Quaternion<T> {
        Quaternion::new(
            self.x.max(other.clone()),
            self.y.max(other.clone()),
            self.z.max(other.clone()),
            self.w.max(other),
        )
    }
}

impl<T: Max> Max for Quaternion<T> {
    fn max(self, other: Self) -> Self {
        self.max_v(other)
    }
}
