/// Rounds a value up to the nearest integer
pub trait Ceil {
    /// Rounds a value up to the nearest integer
    fn ceil(self) -> Self;
}

impl Ceil for f32 {
    fn ceil(self) -> Self {
        f32::ceil(self)
    }
}

impl Ceil for f64 {
    fn ceil(self) -> Self {
        f64::ceil(self)
    }
}
