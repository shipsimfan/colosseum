mod display;
mod from;
mod into;
mod new;

/// A rational number representing an exact fraction
#[derive(Debug, Default, Clone, Copy, Hash)]
pub struct Rational<T> {
    /// The top part of the fraction
    pub numerator: T,

    /// The bottom part of the fraction
    pub denominator: T,
}
