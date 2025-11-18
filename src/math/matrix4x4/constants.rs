use crate::math::{
    Matrix4x4,
    number::{Infinity, NaN, NegInfinity, One, Zero},
};

impl<T: Zero> Matrix4x4<T> {
    /// A matrix of all zeroes
    pub const ZERO: Self = Matrix4x4::zero();
}

impl<T: Zero> Zero for Matrix4x4<T> {
    const ZERO: Self = Self::ZERO;
}

impl<T: One> Matrix4x4<T> {
    /// Matrix of all ones
    pub const ONE: Self = Matrix4x4::one();
}

impl<T: One> One for Matrix4x4<T> {
    const ONE: Self = Self::ONE;
}

impl<T: Zero + One> Matrix4x4<T> {
    /// The matrix which has no effect
    pub const IDENTITY: Self = Self::identity();
}

impl<T: Infinity> Matrix4x4<T> {
    /// Matrix of all infinities (∞)
    pub const INFINITY: Self = Matrix4x4::infinity();
}

impl<T: Infinity> Infinity for Matrix4x4<T> {
    const INFINITY: Self = Matrix4x4::infinity();
}

impl<T: NegInfinity> Matrix4x4<T> {
    /// Matrix of all negative infinities (-∞)
    pub const NEG_INFINITY: Self = Matrix4x4::neg_infinity();
}

impl<T: NegInfinity> NegInfinity for Matrix4x4<T> {
    const NEG_INFINITY: Self = Matrix4x4::neg_infinity();
}

impl<T: NaN> Matrix4x4<T> {
    /// Matrix of all NaN
    pub const NAN: Self = Matrix4x4::nan();
}

impl<T: NaN> NaN for Matrix4x4<T> {
    const NAN: Self = Matrix4x4::nan();
}
