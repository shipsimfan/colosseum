use crate::math::{number::Floor, Vector4};

impl<T: Floor> Vector4<T> {
    /// Rounds the values of [`Vector4`] component wise down to the nearest integer
    pub fn floor(self) -> Self {
        Vector4::new(
            self.x.floor(),
            self.y.floor(),
            self.z.floor(),
            self.w.floor(),
        )
    }
}

impl<T: Floor> Floor for Vector4<T> {
    fn floor(self) -> Self {
        self.floor()
    }
}
