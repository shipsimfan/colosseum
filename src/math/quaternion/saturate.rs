use crate::math::{
    number::{Max, Min, One, Zero},
    Quaternion,
};

impl<T: Min + Max + Zero + One + Clone> Quaternion<T> {
    /// Saturates (clamps between 0.0 and 1.0) the values of a [`Quaternion`] component wise
    pub fn saturate(self) -> Self {
        self.clamp(T::ZERO, T::ONE)
    }
}
