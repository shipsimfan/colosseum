use crate::math::{
    number::{Infinity, NaN, NegInfinity, One, Zero},
    Vector4,
};

impl<T> Vector4<T> {
    /// Create a new [`Vector4`]
    pub const fn new(x: T, y: T, z: T, w: T) -> Self {
        Vector4 { x, y, z, w }
    }

    /// Create a new [`Vector4`] from an array
    pub fn from_array([x, y, z, w]: [T; 4]) -> Self {
        Vector4::new(x, y, z, w)
    }
}

impl<T: Clone> Vector4<T> {
    /// Create a new [`Vector4`] from a slice
    pub fn from_slice(s: &[T]) -> Self {
        assert!(s.len() >= 4);
        Vector4::new(s[0].clone(), s[1].clone(), s[2].clone(), s[3].clone())
    }

    /// Create a new [`Vector4`] consisting of the same values
    pub fn splat(v: T) -> Self {
        Vector4::new(v.clone(), v.clone(), v.clone(), v)
    }
}

impl<T: Zero> Vector4<T> {
    /// Create a new [`Vector4`] containing only zeroes
    pub const fn zero() -> Self {
        Vector4::new(T::ZERO, T::ZERO, T::ZERO, T::ZERO)
    }
}

impl<T: One> Vector4<T> {
    /// Create a new [`Vector4`] containing only ones
    pub const fn one() -> Self {
        Vector4::new(T::ONE, T::ONE, T::ONE, T::ONE)
    }
}

impl<T: Zero + One> Vector4<T> {
    /// Create a new unit [`Vector4`] along the positive x-axis
    pub const fn unit_x() -> Self {
        Vector4::new(T::ONE, T::ZERO, T::ZERO, T::ZERO)
    }

    /// Create a new unit [`Vector4`] along the positive y-axis
    pub const fn unit_y() -> Self {
        Vector4::new(T::ZERO, T::ONE, T::ZERO, T::ZERO)
    }

    /// Create a new unit [`Vector4`] along the positive z-axis
    pub const fn unit_z() -> Self {
        Vector4::new(T::ZERO, T::ZERO, T::ONE, T::ZERO)
    }

    /// Create a new unit [`Vector4`] along the positive w-axis
    pub const fn unit_w() -> Self {
        Vector4::new(T::ZERO, T::ZERO, T::ZERO, T::ONE)
    }
}

impl<T: Infinity> Vector4<T> {
    /// Create a new [`Vector4`] containing only infinities (∞)
    pub const fn infinity() -> Self {
        Vector4::new(T::INFINITY, T::INFINITY, T::INFINITY, T::INFINITY)
    }
}

impl<T: NegInfinity> Vector4<T> {
    /// Create a new [`Vector4`] containing only negative infinities (-∞)
    pub const fn neg_infinity() -> Self {
        Vector4::new(
            T::NEG_INFINITY,
            T::NEG_INFINITY,
            T::NEG_INFINITY,
            T::NEG_INFINITY,
        )
    }
}

impl<T: NaN> Vector4<T> {
    /// Create a new [`Vector4`] containing only `NaN` values
    pub const fn nan() -> Self {
        Vector4::new(T::NAN, T::NAN, T::NAN, T::NAN)
    }
}
