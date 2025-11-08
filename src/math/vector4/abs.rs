use crate::math::{number::Absolute, Vector4};

impl<T: Absolute> Vector4<T> {
    /// Gets the aboslute value of a [`Vector4`] component wise
    pub fn abs(self) -> Self {
        Vector4::new(self.x.abs(), self.y.abs(), self.z.abs(), self.w.abs())
    }
}

impl<T: Absolute> Absolute for Vector4<T> {
    fn abs(self) -> Self {
        self.abs()
    }
}
