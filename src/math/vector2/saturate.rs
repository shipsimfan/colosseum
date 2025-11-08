use crate::math::{
    number::{Max, Min, One, Zero},
    Vector2,
};

impl<T: Min + Max + Zero + One + Clone> Vector2<T> {
    /// Saturates (clamps between 0.0 and 1.0) the values of a [`Vector2`] component wise
    pub fn saturate(self) -> Self {
        self.clamp(T::ZERO, T::ONE)
    }
}
