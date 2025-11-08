use crate::math::{
    number::{Infinity, NaN, NegInfinity, One, Zero},
    Vector3,
};

impl<T> Vector3<T> {
    /// Create a new [`Vector3`]
    pub const fn new(x: T, y: T, z: T) -> Self {
        Vector3 { x, y, z }
    }

    /// Create a new [`Vector3`] from an array
    pub fn from_array([x, y, z]: [T; 3]) -> Self {
        Vector3::new(x, y, z)
    }
}

impl<T: Clone> Vector3<T> {
    /// Create a new [`Vector3`] from a slice
    pub fn from_slice(s: &[T]) -> Self {
        assert!(s.len() >= 3);
        Vector3::new(s[0].clone(), s[1].clone(), s[2].clone())
    }

    /// Create a new [`Vector3`] consisting of the same values
    pub fn splat(v: T) -> Self {
        Vector3::new(v.clone(), v.clone(), v)
    }
}

impl<T: Zero> Vector3<T> {
    /// Create a new [`Vector3`] containing only zeroes
    pub const fn zero() -> Self {
        Vector3::new(T::ZERO, T::ZERO, T::ZERO)
    }
}

impl<T: One> Vector3<T> {
    /// Create a new [`Vector3`] containing only ones
    pub const fn one() -> Self {
        Vector3::new(T::ONE, T::ONE, T::ONE)
    }
}

impl<T: Zero + One> Vector3<T> {
    /// Create a new unit [`Vector3`] along the positive x-axis
    pub const fn unit_x() -> Self {
        Vector3::new(T::ONE, T::ZERO, T::ZERO)
    }

    /// Create a new unit [`Vector3`] along the positive y-axis
    pub const fn unit_y() -> Self {
        Vector3::new(T::ZERO, T::ONE, T::ZERO)
    }

    /// Create a new unit [`Vector3`] along the positive z-axis
    pub const fn unit_z() -> Self {
        Vector3::new(T::ZERO, T::ZERO, T::ONE)
    }
}

impl<T: Infinity> Vector3<T> {
    /// Create a new [`Vector3`] containing only infinities (∞)
    pub const fn infinity() -> Self {
        Vector3::new(T::INFINITY, T::INFINITY, T::INFINITY)
    }
}

impl<T: NegInfinity> Vector3<T> {
    /// Create a new [`Vector3`] containing only negative infinities (-∞)
    pub const fn neg_infinity() -> Self {
        Vector3::new(T::NEG_INFINITY, T::NEG_INFINITY, T::NEG_INFINITY)
    }
}

impl<T: NaN> Vector3<T> {
    /// Create a new [`Vector3`] containing only `NaN` values
    pub const fn nan() -> Self {
        Vector3::new(T::NAN, T::NAN, T::NAN)
    }
}
