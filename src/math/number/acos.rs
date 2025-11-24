/// Computes the acosine of a value in radians
pub trait Acos {
    /// Computes the inverse cosine of a value in radians
    fn acos(self) -> Self;
}

impl Acos for f32 {
    fn acos(self) -> Self {
        f32::acos(self)
    }
}

impl Acos for f64 {
    fn acos(self) -> Self {
        f64::acos(self)
    }
}
