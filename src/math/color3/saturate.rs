use crate::math::{
    number::{Max, Min, One, Zero},
    Color3,
};

impl<T: Min + Max + Zero + One + Clone> Color3<T> {
    /// Saturates (clamps between 0.0 and 1.0) the values of a [`Color3`] component wise
    pub fn saturate(self) -> Self {
        self.clamp(T::ZERO, T::ONE)
    }
}
