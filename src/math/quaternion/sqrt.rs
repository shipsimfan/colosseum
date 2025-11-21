use crate::math::{number::Sqrt, Quaternion};

impl<T: Sqrt> Quaternion<T> {
    /// Gets the component-wise square root
    pub fn sqrt(self) -> Self {
        Quaternion::new(self.x.sqrt(), self.y.sqrt(), self.z.sqrt(), self.w.sqrt())
    }
}

impl<T: Sqrt> Sqrt for Quaternion<T> {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
