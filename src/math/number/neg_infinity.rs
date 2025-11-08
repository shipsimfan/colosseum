/// Defines a infinity (∞) value for a type
pub trait NegInfinity {
    /// The infinity (∞) value of this type
    const NEG_INFINITY: Self;
}

impl NegInfinity for f32 {
    const NEG_INFINITY: Self = f32::NEG_INFINITY;
}

impl NegInfinity for f64 {
    const NEG_INFINITY: Self = f64::NEG_INFINITY;
}
