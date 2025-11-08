use crate::math::{number::Min, Color3};

impl<T: Min> Color3<T> {
    /// Compares and returns the minimum of two vectors component-wise
    pub fn min_v(self, other: Color3<T>) -> Color3<T> {
        Color3::new(
            self.r.min(other.r),
            self.g.min(other.g),
            self.b.min(other.b),
        )
    }
}

impl<T: Min + Clone> Color3<T> {
    /// Compares and returns the minimum of a vector component-wise and a scalar
    pub fn min(self, other: T) -> Color3<T> {
        Color3::new(
            self.r.min(other.clone()),
            self.g.min(other.clone()),
            self.b.min(other),
        )
    }
}

impl<T: Min> Min for Color3<T> {
    fn min(self, other: Self) -> Self {
        self.min_v(other)
    }
}
