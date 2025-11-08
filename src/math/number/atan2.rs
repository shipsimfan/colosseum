/// Computes the four quadrant arctangent of the two values in radians
pub trait Atan2 {
    /// Computes the four quadrant arctangent of the two values in radians
    fn atan2(self, other: Self) -> Self;
}

impl Atan2 for f32 {
    fn atan2(self, other: Self) -> Self {
        f32::atan2(self, other)
    }
}

impl Atan2 for f64 {
    fn atan2(self, other: Self) -> Self {
        f64::atan2(self, other)
    }
}
