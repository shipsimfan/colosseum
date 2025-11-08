use crate::math::{number::Round, Vector4};

impl<T: Round> Vector4<T> {
    /// Rounds the values of [`Vector4`] component wise to the nearest integer
    pub fn round(self) -> Self {
        Vector4::new(
            self.x.round(),
            self.y.round(),
            self.z.round(),
            self.w.round(),
        )
    }
}

impl<T: Round> Round for Vector4<T> {
    fn round(self) -> Self {
        self.round()
    }
}
