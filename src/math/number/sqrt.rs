/// Finds the square root of a value
pub trait Sqrt {
    /// Finds the square root of a value
    fn sqrt(self) -> Self;
}

impl Sqrt for f32 {
    fn sqrt(self) -> Self {
        f32::sqrt(self)
    }
}

impl Sqrt for f64 {
    fn sqrt(self) -> Self {
        f64::sqrt(self)
    }
}
