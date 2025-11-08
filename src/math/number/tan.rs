/// Computes the tangent of a value in radians
pub trait Tan {
    /// Computes the tangent of a value in radians
    fn tan(self) -> Self;
}

impl Tan for f32 {
    fn tan(self) -> Self {
        f32::tan(self)
    }
}

impl Tan for f64 {
    fn tan(self) -> Self {
        f64::tan(self)
    }
}
