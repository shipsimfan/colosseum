use crate::math::{number::Ceil, Vector3};

impl<T: Ceil> Vector3<T> {
    /// Rounds the values of [`Vector3`] component wise up to the nearest integer
    pub fn ceil(self) -> Self {
        Vector3::new(self.x.ceil(), self.y.ceil(), self.z.ceil())
    }
}

impl<T: Ceil> Ceil for Vector3<T> {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
