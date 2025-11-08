use crate::math::{number::Floor, Vector3};

impl<T: Floor> Vector3<T> {
    /// Rounds the values of [`Vector3`] component wise down to the nearest integer
    pub fn floor(self) -> Self {
        Vector3::new(self.x.floor(), self.y.floor(), self.z.floor())
    }
}

impl<T: Floor> Floor for Vector3<T> {
    fn floor(self) -> Self {
        self.floor()
    }
}
