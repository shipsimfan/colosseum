use crate::math::{number::IntoF32, Vector2};

impl<T: IntoF32> Vector2<T> {
    /// Convert this [`Vector2<T>`] into a [`Vector2<f32>`]
    pub fn into_f32(self) -> Vector2<f32> {
        Vector2::new(self.x.into_f32(), self.y.into_f32())
    }
}
