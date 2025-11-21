use crate::math::{number::Fract, Quaternion};

impl<T: Fract> Quaternion<T> {
    /// Gets the fractional parts of the values of a [`Quaternion`] component-wise
    pub fn fract(self) -> Self {
        Quaternion::new(
            self.x.fract(),
            self.y.fract(),
            self.z.fract(),
            self.w.fract(),
        )
    }
}

impl<T: Fract> Fract for Quaternion<T> {
    fn fract(self) -> Self {
        self.fract()
    }
}
