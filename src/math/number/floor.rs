/// Rounds a value down to the nearest integer
pub trait Floor {
    /// Rounds a value down to the nearest integer
    fn floor(self) -> Self;
}

impl Floor for f32 {
    fn floor(self) -> Self {
        f32::floor(self)
    }
}

impl Floor for f64 {
    fn floor(self) -> Self {
        f64::floor(self)
    }
}
