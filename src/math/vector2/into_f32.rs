use crate::math::{Vector2, Vector2f, number::IntoF32};

impl<T: IntoF32> Vector2<T> {
    /// Convert this [`Vector2<T>`] into a [`Vector2f`]
    pub fn into_f32(self) -> Vector2f {
        Vector2::new(self.x.into_f32(), self.y.into_f32())
    }
}
