use crate::math::{number::Max, Color3};

impl<T: Max> Color3<T> {
    /// Compares and returns the maximum of two vectors component-wise
    pub fn max_v(self, other: Color3<T>) -> Color3<T> {
        Color3::new(
            self.r.max(other.r),
            self.g.max(other.g),
            self.b.max(other.b),
        )
    }
}

impl<T: Max + Clone> Color3<T> {
    /// Compares and returns the maximum of a vector component-wise and a scalar
    pub fn max(self, other: T) -> Color3<T> {
        Color3::new(
            self.r.max(other.clone()),
            self.g.max(other.clone()),
            self.b.max(other),
        )
    }
}

impl<T: Max> Max for Color3<T> {
    fn max(self, other: Self) -> Self {
        self.max_v(other)
    }
}
