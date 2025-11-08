use crate::math::{
    number::{Infinity, NaN, NegInfinity, One, Zero},
    Vector4,
};

impl<T: Zero> Vector4<T> {
    /// A vector of all zeroes
    pub const ZERO: Self = Vector4::zero();
}

impl<T: Zero> Zero for Vector4<T> {
    const ZERO: Self = Self::ZERO;
}

impl<T: One> Vector4<T> {
    /// Vector of all ones
    pub const ONE: Self = Vector4::one();
}

impl<T: One> One for Vector4<T> {
    const ONE: Self = Self::ONE;
}

impl<T: Zero + One> Vector4<T> {
    /// Unit [`Vector4`] along the positive x-axis
    pub const UNIT_X: Self = Vector4::unit_x();

    /// Unit [`Vector4`] along the positive y-axis
    pub const UNIT_Y: Self = Vector4::unit_y();

    /// Unit [`Vector4`] along the positive z-axis
    pub const UNIT_Z: Self = Vector4::unit_z();

    /// Unit [`Vector4`] along the positive w-axis
    pub const UNIT_W: Self = Vector4::unit_w();
}

impl<T: Infinity> Vector4<T> {
    /// Vector of all infinities (∞)
    pub const INFINITY: Self = Vector4::infinity();
}

impl<T: Infinity> Infinity for Vector4<T> {
    const INFINITY: Self = Vector4::infinity();
}

impl<T: NegInfinity> Vector4<T> {
    /// Vector of all negative infinities (-∞)
    pub const NEG_INFINITY: Self = Vector4::neg_infinity();
}

impl<T: NegInfinity> NegInfinity for Vector4<T> {
    const NEG_INFINITY: Self = Vector4::neg_infinity();
}

impl<T: NaN> Vector4<T> {
    /// Vector of all NaN
    pub const NAN: Self = Vector4::nan();
}

impl<T: NaN> NaN for Vector4<T> {
    const NAN: Self = Vector4::nan();
}
