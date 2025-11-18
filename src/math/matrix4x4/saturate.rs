use crate::math::{
    Matrix4x4,
    number::{Max, Min, One, Zero},
};

impl<T: Min + Max + Zero + One + Clone> Matrix4x4<T> {
    /// Saturates (clamps between 0.0 and 1.0) the values of a [`Matrix4x4`] component wise
    pub fn saturate(self) -> Self {
        self.clamp(T::ZERO, T::ONE)
    }
}
