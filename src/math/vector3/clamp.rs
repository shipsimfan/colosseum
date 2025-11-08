use crate::math::{
    number::{Max, Min},
    Vector3,
};

impl<T: Min + Max> Vector3<T> {
    /// Clamps the values of the vector between two vectors component-wise
    pub fn clamp_v(self, min: Vector3<T>, max: Vector3<T>) -> Vector3<T> {
        Vector3::new(
            self.x.max(min.x).min(max.x),
            self.y.max(min.y).min(max.y),
            self.z.max(min.z).min(max.z),
        )
    }
}

impl<T: Min + Max + Clone> Vector3<T> {
    /// Clamps the values of the vector between two values component-wise
    pub fn clamp(self, min: T, max: T) -> Vector3<T> {
        Vector3::new(
            self.x.max(min.clone()).min(max.clone()),
            self.y.max(min.clone()).min(max.clone()),
            self.z.max(min).min(max),
        )
    }
}
