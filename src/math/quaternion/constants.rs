use crate::math::{
    Quaternion,
    number::{Infinity, NaN, NegInfinity, One, Zero},
};

impl<T: Zero> Quaternion<T> {
    /// A vector of all zeroes
    pub const ZERO: Self = Quaternion::zero();
}

impl<T: Zero> Zero for Quaternion<T> {
    const ZERO: Self = Self::ZERO;
}

impl<T: One> Quaternion<T> {
    /// Vector of all ones
    pub const ONE: Self = Quaternion::one();
}

impl<T: One> One for Quaternion<T> {
    const ONE: Self = Self::ONE;
}

impl<T: Zero + One> Quaternion<T> {
    /// Unit [`Quaternion`] along the positive x-axis
    pub const UNIT_X: Self = Quaternion::unit_x();

    /// Unit [`Quaternion`] along the positive y-axis
    pub const UNIT_Y: Self = Quaternion::unit_y();

    /// Unit [`Quaternion`] along the positive z-axis
    pub const UNIT_Z: Self = Quaternion::unit_z();

    /// Unit [`Quaternion`] along the positive w-axis
    pub const UNIT_W: Self = Quaternion::unit_w();

    /// A [`Quaternion`] that represents no rotation
    pub const IDENTITY: Self = Quaternion::identity();
}

impl<T: Infinity> Quaternion<T> {
    /// Vector of all infinities (∞)
    pub const INFINITY: Self = Quaternion::infinity();
}

impl<T: Infinity> Infinity for Quaternion<T> {
    const INFINITY: Self = Quaternion::infinity();
}

impl<T: NegInfinity> Quaternion<T> {
    /// Vector of all negative infinities (-∞)
    pub const NEG_INFINITY: Self = Quaternion::neg_infinity();
}

impl<T: NegInfinity> NegInfinity for Quaternion<T> {
    const NEG_INFINITY: Self = Quaternion::neg_infinity();
}

impl<T: NaN> Quaternion<T> {
    /// Vector of all NaN
    pub const NAN: Self = Quaternion::nan();
}

impl<T: NaN> NaN for Quaternion<T> {
    const NAN: Self = Quaternion::nan();
}
