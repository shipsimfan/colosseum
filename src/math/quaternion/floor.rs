use crate::math::{number::Floor, Quaternion};

impl<T: Floor> Quaternion<T> {
    /// Rounds the values of [`Quaternion`] component wise down to the nearest integer
    pub fn floor(self) -> Self {
        Quaternion::new(
            self.x.floor(),
            self.y.floor(),
            self.z.floor(),
            self.w.floor(),
        )
    }
}

impl<T: Floor> Floor for Quaternion<T> {
    fn floor(self) -> Self {
        self.floor()
    }
}
