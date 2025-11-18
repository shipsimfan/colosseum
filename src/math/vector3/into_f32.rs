use crate::math::{Vector3, Vector3f, number::IntoF32};

impl<T: IntoF32> Vector3<T> {
    /// Convert this [`Vector3<T>`] into a [`Vector3f`]
    pub fn into_f32(self) -> Vector3f {
        Vector3::new(self.x.into_f32(), self.y.into_f32(), self.z.into_f32())
    }
}
