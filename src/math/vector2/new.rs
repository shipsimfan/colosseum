use crate::math::{
    number::{Infinity, NaN, NegInfinity, One, Zero},
    Vector2,
};

impl<T> Vector2<T> {
    /// Create a new [`Vector2`]
    pub const fn new(x: T, y: T) -> Self {
        Vector2 { x, y }
    }

    /// Create a new [`Vector2`] from an array
    pub fn from_array([x, y]: [T; 2]) -> Self {
        Vector2::new(x, y)
    }
}

impl<T: Clone> Vector2<T> {
    /// Create a new [`Vector2`] from a slice
    pub fn from_slice(s: &[T]) -> Self {
        assert!(s.len() >= 2);
        Vector2::new(s[0].clone(), s[1].clone())
    }

    /// Create a new [`Vector2`] consisting of the same values
    pub fn splat(v: T) -> Self {
        Vector2::new(v.clone(), v)
    }
}

impl<T: Zero> Vector2<T> {
    /// Create a new [`Vector2`] containing only zeroes
    pub const fn zero() -> Self {
        Vector2::new(T::ZERO, T::ZERO)
    }
}

impl<T: One> Vector2<T> {
    /// Create a new [`Vector2`] containing only ones
    pub const fn one() -> Self {
        Vector2::new(T::ONE, T::ONE)
    }
}

impl<T: Zero + One> Vector2<T> {
    /// Create a new unit [`Vector2`] along the positive x-axis
    pub const fn unit_x() -> Self {
        Vector2::new(T::ONE, T::ZERO)
    }

    /// Create a new unit [`Vector2`] along the positive y-axis
    pub const fn unit_y() -> Self {
        Vector2::new(T::ZERO, T::ONE)
    }
}

impl<T: Infinity> Vector2<T> {
    /// Create a new [`Vector2`] containing only infinities (∞)
    pub const fn infinity() -> Self {
        Vector2::new(T::INFINITY, T::INFINITY)
    }
}

impl<T: NegInfinity> Vector2<T> {
    /// Create a new [`Vector2`] containing only negative infinities (-∞)
    pub const fn neg_infinity() -> Self {
        Vector2::new(T::NEG_INFINITY, T::NEG_INFINITY)
    }
}

impl<T: NaN> Vector2<T> {
    /// Create a new [`Vector2`] containing only `NaN` values
    pub const fn nan() -> Self {
        Vector2::new(T::NAN, T::NAN)
    }
}
