/// Rounds a value to the nearest integer
pub trait Round {
    /// Rounds a value to the nearest integer
    fn round(self) -> Self;
}

impl Round for f32 {
    fn round(self) -> Self {
        f32::round(self)
    }
}

impl Round for f64 {
    fn round(self) -> Self {
        f64::round(self)
    }
}
