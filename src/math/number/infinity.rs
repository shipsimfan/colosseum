/// Defines a infinity (∞) value for a type
pub trait Infinity {
    /// The infinity (∞) value of this type
    const INFINITY: Self;
}

impl Infinity for f32 {
    const INFINITY: Self = f32::INFINITY;
}

impl Infinity for f64 {
    const INFINITY: Self = f64::INFINITY;
}
