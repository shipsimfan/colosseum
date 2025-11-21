use crate::math::{number::Round, Quaternion};

impl<T: Round> Quaternion<T> {
    /// Rounds the values of [`Quaternion`] component wise to the nearest integer
    pub fn round(self) -> Self {
        Quaternion::new(
            self.x.round(),
            self.y.round(),
            self.z.round(),
            self.w.round(),
        )
    }
}

impl<T: Round> Round for Quaternion<T> {
    fn round(self) -> Self {
        self.round()
    }
}
