use crate::math::{number::Ceil, Color3};

impl<T: Ceil> Color3<T> {
    /// Rounds the values of [`Color3`] component wise up to the nearest integer
    pub fn ceil(self) -> Self {
        Color3::new(self.r.ceil(), self.g.ceil(), self.b.ceil())
    }
}

impl<T: Ceil> Ceil for Color3<T> {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
