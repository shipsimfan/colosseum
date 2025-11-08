use crate::math::{number::Fract, Vector3};

impl<T: Fract> Vector3<T> {
    /// Gets the fractional parts of the values of a [`Vector3`] component-wise
    pub fn fract(self) -> Self {
        Vector3::new(self.x.fract(), self.y.fract(), self.z.fract())
    }
}

impl<T: Fract> Fract for Vector3<T> {
    fn fract(self) -> Self {
        self.fract()
    }
}
