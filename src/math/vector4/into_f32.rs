use crate::math::{number::IntoF32, Vector4};

impl<T: IntoF32> Vector4<T> {
    /// Convert this [`Vector4<T>`] into a [`Vector4<f32>`]
    pub fn into_f32(self) -> Vector4<f32> {
        Vector4::new(
            self.x.into_f32(),
            self.y.into_f32(),
            self.z.into_f32(),
            self.w.into_f32(),
        )
    }
}
