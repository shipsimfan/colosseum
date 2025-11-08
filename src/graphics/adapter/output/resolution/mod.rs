use crate::math::{Rational, Vector2u};

mod enumerate;
mod get;

/// A resolution an output supports
pub struct OutputResolution {
    /// The size of the display at this resolution
    size: Vector2u,

    /// The supported refresh rates at this resolution
    refresh_rates: Vec<Rational<u32>>,
}
