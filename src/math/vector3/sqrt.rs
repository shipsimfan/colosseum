use crate::math::{number::Sqrt, Vector3};

impl<T: Sqrt> Vector3<T> {
    /// Gets the component-wise square root
    pub fn sqrt(self) -> Self {
        Vector3::new(self.x.sqrt(), self.y.sqrt(), self.z.sqrt())
    }
}

impl<T: Sqrt> Sqrt for Vector3<T> {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
