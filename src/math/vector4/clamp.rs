use crate::math::{
    number::{Max, Min},
    Vector4,
};

impl<T: Min + Max> Vector4<T> {
    /// Clamps the values of the vector between two vectors component-wise
    pub fn clamp_v(self, min: Vector4<T>, max: Vector4<T>) -> Vector4<T> {
        Vector4::new(
            self.x.max(min.x).min(max.x),
            self.y.max(min.y).min(max.y),
            self.z.max(min.z).min(max.z),
            self.w.max(min.w).min(max.w),
        )
    }
}

impl<T: Min + Max + Clone> Vector4<T> {
    /// Clamps the values of the vector between two values component-wise
    pub fn clamp(self, min: T, max: T) -> Vector4<T> {
        Vector4::new(
            self.x.max(min.clone()).min(max.clone()),
            self.y.max(min.clone()).min(max.clone()),
            self.z.max(min.clone()).min(max.clone()),
            self.w.max(min).min(max),
        )
    }
}
