use crate::math::{
    number::{Max, Min},
    Vector2,
};

impl<T: Min + Max> Vector2<T> {
    /// Clamps the values of the vector between two vectors component-wise
    pub fn clamp_v(self, min: Vector2<T>, max: Vector2<T>) -> Vector2<T> {
        Vector2::new(self.x.max(min.x).min(max.x), self.y.max(min.y).min(max.y))
    }
}

impl<T: Min + Max + Clone> Vector2<T> {
    /// Clamps the values of the vector between two values component-wise
    pub fn clamp(self, min: T, max: T) -> Vector2<T> {
        Vector2::new(
            self.x.max(min.clone()).min(max.clone()),
            self.y.max(min).min(max),
        )
    }
}
