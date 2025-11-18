use crate::math::{Vector4, Vector4f, number::IntoF32};

impl<T: IntoF32> Vector4<T> {
    /// Convert this [`Vector4<T>`] into a [`Vector4f`]
    pub fn into_f32(self) -> Vector4f {
        Vector4::new(
            self.x.into_f32(),
            self.y.into_f32(),
            self.z.into_f32(),
            self.w.into_f32(),
        )
    }
}
