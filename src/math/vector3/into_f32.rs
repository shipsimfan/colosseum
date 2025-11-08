use crate::math::{number::IntoF32, Vector3};

impl<T: IntoF32> Vector3<T> {
    /// Convert this [`Vector3<T>`] into a [`Vector3<f32>`]
    pub fn into_f32(self) -> Vector3<f32> {
        Vector3::new(self.x.into_f32(), self.y.into_f32(), self.z.into_f32())
    }
}
