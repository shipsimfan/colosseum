/// Computes the cosine of a value in radians
pub trait Cos {
    /// Computes the cosine of a value in radians
    fn cos(self) -> Self;
}

impl Cos for f32 {
    fn cos(self) -> Self {
        f32::cos(self)
    }
}

impl Cos for f64 {
    fn cos(self) -> Self {
        f64::cos(self)
    }
}
