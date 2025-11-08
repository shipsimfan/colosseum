use crate::math::{number::IntoF32, Color3};

impl<T: IntoF32> Color3<T> {
    /// Convert this [`Color3<T>`] into a [`Color3<f32>`]
    pub fn into_f32(self) -> Color3<f32> {
        Color3::new(self.r.into_f32(), self.g.into_f32(), self.b.into_f32())
    }
}
