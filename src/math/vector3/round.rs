use crate::math::{number::Round, Vector3};

impl<T: Round> Vector3<T> {
    /// Rounds the values of [`Vector3`] component wise to the nearest integer
    pub fn round(self) -> Self {
        Vector3::new(self.x.round(), self.y.round(), self.z.round())
    }
}

impl<T: Round> Round for Vector3<T> {
    fn round(self) -> Self {
        self.round()
    }
}
