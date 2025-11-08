/// Defines a value representing "Not a Number"
pub trait NaN {
    /// The value which represents "Not a Number"
    const NAN: Self;
}

impl NaN for f32 {
    const NAN: Self = f32::NAN;
}

impl NaN for f64 {
    const NAN: Self = f64::NAN;
}
