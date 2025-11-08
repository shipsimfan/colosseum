use crate::math::{number::Ceil, Vector4};

impl<T: Ceil> Vector4<T> {
    /// Rounds the values of [`Vector4`] component wise up to the nearest integer
    pub fn ceil(self) -> Self {
        Vector4::new(self.x.ceil(), self.y.ceil(), self.z.ceil(), self.w.ceil())
    }
}

impl<T: Ceil> Ceil for Vector4<T> {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
