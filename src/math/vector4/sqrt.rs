use crate::math::{number::Sqrt, Vector4};

impl<T: Sqrt> Vector4<T> {
    /// Gets the component-wise square root
    pub fn sqrt(self) -> Self {
        Vector4::new(self.x.sqrt(), self.y.sqrt(), self.z.sqrt(), self.w.sqrt())
    }
}

impl<T: Sqrt> Sqrt for Vector4<T> {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
