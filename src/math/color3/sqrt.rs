use crate::math::{number::Sqrt, Color3};

impl<T: Sqrt> Color3<T> {
    /// Gets the component-wise square root
    pub fn sqrt(self) -> Self {
        Color3::new(self.r.sqrt(), self.g.sqrt(), self.b.sqrt())
    }
}

impl<T: Sqrt> Sqrt for Color3<T> {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
