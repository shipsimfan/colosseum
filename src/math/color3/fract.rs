use crate::math::{number::Fract, Color3};

impl<T: Fract> Color3<T> {
    /// Gets the fractional parts of the values of a [`Color3`] component-wise
    pub fn fract(self) -> Self {
        Color3::new(self.r.fract(), self.g.fract(), self.b.fract())
    }
}

impl<T: Fract> Fract for Color3<T> {
    fn fract(self) -> Self {
        self.fract()
    }
}
