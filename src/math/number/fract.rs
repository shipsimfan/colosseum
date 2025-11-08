/// Gets the fractional part of a floating point number
pub trait Fract {
    /// Gets the fractional part of a floating point number
    fn fract(self) -> Self;
}

impl Fract for f32 {
    fn fract(self) -> Self {
        f32::fract(self)
    }
}

impl Fract for f64 {
    fn fract(self) -> Self {
        f64::fract(self)
    }
}
