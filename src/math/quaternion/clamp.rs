use crate::math::{
    Quaternion,
    number::{Max, Min},
};

impl<T: Min + Max> Quaternion<T> {
    /// Clamps the values of the vector between two vectors component-wise
    pub fn clamp_v(self, min: Quaternion<T>, max: Quaternion<T>) -> Quaternion<T> {
        Quaternion::new(
            self.x.max(min.x).min(max.x),
            self.y.max(min.y).min(max.y),
            self.z.max(min.z).min(max.z),
            self.w.max(min.w).min(max.w),
        )
    }
}

impl<T: Min + Max + Clone> Quaternion<T> {
    /// Clamps the values of the vector between two values component-wise
    pub fn clamp(self, min: T, max: T) -> Quaternion<T> {
        Quaternion::new(
            self.x.max(min.clone()).min(max.clone()),
            self.y.max(min.clone()).min(max.clone()),
            self.z.max(min.clone()).min(max.clone()),
            self.w.max(min).min(max),
        )
    }
}
