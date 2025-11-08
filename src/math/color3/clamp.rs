use crate::math::{
    number::{Max, Min},
    Color3,
};

impl<T: Min + Max> Color3<T> {
    /// Clamps the values of the vector between two vectors component-wise
    pub fn clamp_v(self, min: Color3<T>, max: Color3<T>) -> Color3<T> {
        Color3::new(
            self.r.max(min.r).min(max.r),
            self.g.max(min.g).min(max.g),
            self.b.max(min.b).min(max.b),
        )
    }
}

impl<T: Min + Max + Clone> Color3<T> {
    /// Clamps the values of the vector between two values component-wise
    pub fn clamp(self, min: T, max: T) -> Color3<T> {
        Color3::new(
            self.r.max(min.clone()).min(max.clone()),
            self.g.max(min.clone()).min(max.clone()),
            self.b.max(min).min(max),
        )
    }
}
