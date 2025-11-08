use crate::math::{number::Absolute, Vector3};

impl<T: Absolute> Vector3<T> {
    /// Gets the aboslute value of a [`Vector3`] component wise
    pub fn abs(self) -> Self {
        Vector3::new(self.x.abs(), self.y.abs(), self.z.abs())
    }
}

impl<T: Absolute> Absolute for Vector3<T> {
    fn abs(self) -> Self {
        self.abs()
    }
}
