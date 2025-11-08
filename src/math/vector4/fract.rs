use crate::math::{number::Fract, Vector4};

impl<T: Fract> Vector4<T> {
    /// Gets the fractional parts of the values of a [`Vector4`] component-wise
    pub fn fract(self) -> Self {
        Vector4::new(
            self.x.fract(),
            self.y.fract(),
            self.z.fract(),
            self.w.fract(),
        )
    }
}

impl<T: Fract> Fract for Vector4<T> {
    fn fract(self) -> Self {
        self.fract()
    }
}
