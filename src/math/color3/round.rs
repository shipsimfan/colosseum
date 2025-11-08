use crate::math::{number::Round, Color3};

impl<T: Round> Color3<T> {
    /// Rounds the values of [`Color3`] component wise to the nearest integer
    pub fn round(self) -> Self {
        Color3::new(self.r.round(), self.g.round(), self.b.round())
    }
}

impl<T: Round> Round for Color3<T> {
    fn round(self) -> Self {
        self.round()
    }
}
