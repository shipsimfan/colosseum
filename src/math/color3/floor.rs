use crate::math::{number::Floor, Color3};

impl<T: Floor> Color3<T> {
    /// Rounds the values of [`Color3`] component wise down to the nearest integer
    pub fn floor(self) -> Self {
        Color3::new(self.r.floor(), self.g.floor(), self.b.floor())
    }
}

impl<T: Floor> Floor for Color3<T> {
    fn floor(self) -> Self {
        self.floor()
    }
}
