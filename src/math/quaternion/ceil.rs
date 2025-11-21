use crate::math::{Quaternion, number::Ceil};

impl<T: Ceil> Quaternion<T> {
    /// Rounds the values of [`Quaternion`] component wise up to the nearest integer
    pub fn ceil(self) -> Self {
        Quaternion::new(self.x.ceil(), self.y.ceil(), self.z.ceil(), self.w.ceil())
    }
}

impl<T: Ceil> Ceil for Quaternion<T> {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
