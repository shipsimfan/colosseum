use crate::math::{Quaternion, Quaternionf, number::IntoF32};

impl<T: IntoF32> Quaternion<T> {
    /// Convert this [`Quaternion<T>`] into a [`Quaternionf`]
    pub fn into_f32(self) -> Quaternionf {
        Quaternion::new(
            self.x.into_f32(),
            self.y.into_f32(),
            self.z.into_f32(),
            self.w.into_f32(),
        )
    }
}
