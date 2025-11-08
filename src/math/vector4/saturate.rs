use crate::math::{
    number::{Max, Min, One, Zero},
    Vector4,
};

impl<T: Min + Max + Zero + One + Clone> Vector4<T> {
    /// Saturates (clamps between 0.0 and 1.0) the values of a [`Vector4`] component wise
    pub fn saturate(self) -> Self {
        self.clamp(T::ZERO, T::ONE)
    }
}
