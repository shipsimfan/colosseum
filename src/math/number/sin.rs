/// Computes the sine of a value in radians
pub trait Sin {
    /// Computes the sine of a value in radians
    fn sin(self) -> Self;
}

impl Sin for f32 {
    fn sin(self) -> Self {
        f32::sin(self)
    }
}

impl Sin for f64 {
    fn sin(self) -> Self {
        f64::sin(self)
    }
}
