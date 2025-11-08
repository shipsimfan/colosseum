use crate::math::{
    number::{One, Zero},
    Rational,
};

impl<T> Rational<T> {
    /// Create a new [`Rational`]
    pub const fn new(numerator: T, denominator: T) -> Self {
        Rational {
            numerator,
            denominator,
        }
    }
    /// Create a new [`Rational`] from an array
    pub fn from_array([x, y]: [T; 2]) -> Self {
        Rational::new(x, y)
    }
}

impl<T: Clone> Rational<T> {
    /// Create a new [`Rational`] from a slice
    pub fn from_slice(s: &[T]) -> Self {
        Rational::new(s[0].clone(), s[1].clone())
    }
}

impl<T: One> Rational<T> {
    /// Create a new [`Rational`] for an integer `value`
    pub const fn integer(value: T) -> Self {
        Rational {
            numerator: value,
            denominator: T::ONE,
        }
    }

    /// Create a new [`Rational`] equal to 1
    pub const fn one() -> Self {
        Rational {
            numerator: T::ONE,
            denominator: T::ONE,
        }
    }
}

impl<T: One + Zero> Rational<T> {
    /// Create a new [`Rational`] equal to 0
    pub const fn zero() -> Self {
        Rational {
            numerator: T::ZERO,
            denominator: T::ONE,
        }
    }
}
