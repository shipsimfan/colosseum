use crate::math::{
    number::{One, Zero},
    Color3,
};

impl<T> Color3<T> {
    /// Create a new [`Color3`]
    pub const fn new(r: T, g: T, b: T) -> Self {
        Color3 { r, g, b }
    }

    /// Create a new [`Color3`] from an array
    pub fn from_array([r, g, b]: [T; 3]) -> Self {
        Color3::new(r, g, b)
    }
}

impl<T: Clone> Color3<T> {
    /// Create a new [`Color3`] from a slice
    pub fn from_slice(s: &[T]) -> Self {
        assert!(s.len() >= 3);
        Color3::new(s[0].clone(), s[1].clone(), s[2].clone())
    }

    /// Create a new [`Color3`] consisting of the same values
    pub fn splat(v: T) -> Self {
        Color3::new(v.clone(), v.clone(), v)
    }
}

impl<T: Zero> Color3<T> {
    /// Create a new [`Color3`] representing black
    pub const fn black() -> Self {
        Color3::new(T::ZERO, T::ZERO, T::ZERO)
    }
}

impl<T: One> Color3<T> {
    /// Create a new [`Color3`] representing white
    pub const fn white() -> Self {
        Color3::new(T::ONE, T::ONE, T::ONE)
    }
}

impl<T: Zero + One> Color3<T> {
    /// Create a new unit [`Color3`] representing red
    pub const fn red() -> Self {
        Color3::new(T::ONE, T::ZERO, T::ZERO)
    }

    /// Create a new unit [`Color3`] representing green
    pub const fn green() -> Self {
        Color3::new(T::ZERO, T::ONE, T::ZERO)
    }

    /// Create a new unit [`Color3`] representing blue
    pub const fn blue() -> Self {
        Color3::new(T::ZERO, T::ZERO, T::ONE)
    }
}
