use crate::math::{Quaternion, number::Absolute};

impl<T: Absolute> Quaternion<T> {
    /// Gets the aboslute value of a [`Quaternion`] component wise
    pub fn abs(self) -> Self {
        Quaternion::new(self.x.abs(), self.y.abs(), self.z.abs(), self.w.abs())
    }
}

impl<T: Absolute> Absolute for Quaternion<T> {
    fn abs(self) -> Self {
        self.abs()
    }
}
