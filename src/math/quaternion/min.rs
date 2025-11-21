use crate::math::{number::Min, Quaternion};

impl<T: Min> Quaternion<T> {
    /// Compares and returns the minimum of two vectors component-wise
    pub fn min_v(self, other: Quaternion<T>) -> Quaternion<T> {
        Quaternion::new(
            self.x.min(other.x),
            self.y.min(other.y),
            self.z.min(other.z),
            self.w.min(other.w),
        )
    }
}

impl<T: Min + Clone> Quaternion<T> {
    /// Compares and returns the minimum of a vector component-wise and a scalar
    pub fn min(self, other: T) -> Quaternion<T> {
        Quaternion::new(
            self.x.min(other.clone()),
            self.y.min(other.clone()),
            self.z.min(other.clone()),
            self.w.min(other),
        )
    }
}

impl<T: Min> Min for Quaternion<T> {
    fn min(self, other: Self) -> Self {
        self.min_v(other)
    }
}
