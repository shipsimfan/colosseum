use crate::math::{
    Quaternion, Vector3,
    number::{Cos, Infinity, NaN, NegInfinity, One, Sin, Sqrt, Zero},
};
use std::ops::{Add, Div, DivAssign, Mul, Sub};

impl<T> Quaternion<T> {
    /// Create a new [`Quaternion`]
    pub const fn new(x: T, y: T, z: T, w: T) -> Self {
        Quaternion { x, y, z, w }
    }

    /// Create a new [`Quaternion`] from an array
    pub fn from_array([x, y, z, w]: [T; 4]) -> Self {
        Quaternion::new(x, y, z, w)
    }
}

impl<T: Clone> Quaternion<T> {
    /// Create a new [`Quaternion`] from a slice
    pub fn from_slice(s: &[T]) -> Self {
        assert!(s.len() >= 4);
        Quaternion::new(s[0].clone(), s[1].clone(), s[2].clone(), s[3].clone())
    }

    /// Create a new [`Quaternion`] consisting of the same values
    pub fn splat(v: T) -> Self {
        Quaternion::new(v.clone(), v.clone(), v.clone(), v)
    }
}

impl<T: Zero> Quaternion<T> {
    /// Create a new [`Quaternion`] containing only zeroes
    pub const fn zero() -> Self {
        Quaternion::new(T::ZERO, T::ZERO, T::ZERO, T::ZERO)
    }
}

impl<T: One> Quaternion<T> {
    /// Create a new [`Quaternion`] containing only ones
    pub const fn one() -> Self {
        Quaternion::new(T::ONE, T::ONE, T::ONE, T::ONE)
    }
}

impl<T: Zero + One> Quaternion<T> {
    /// Create a new unit [`Quaternion`] along the positive x-axis
    pub const fn unit_x() -> Self {
        Quaternion::new(T::ONE, T::ZERO, T::ZERO, T::ZERO)
    }

    /// Create a new unit [`Quaternion`] along the positive y-axis
    pub const fn unit_y() -> Self {
        Quaternion::new(T::ZERO, T::ONE, T::ZERO, T::ZERO)
    }

    /// Create a new unit [`Quaternion`] along the positive z-axis
    pub const fn unit_z() -> Self {
        Quaternion::new(T::ZERO, T::ZERO, T::ONE, T::ZERO)
    }

    /// Create a new unit [`Quaternion`] along the positive w-axis
    pub const fn unit_w() -> Self {
        Quaternion::new(T::ZERO, T::ZERO, T::ZERO, T::ONE)
    }
}

impl<T: Infinity> Quaternion<T> {
    /// Create a new [`Quaternion`] containing only infinities (∞)
    pub const fn infinity() -> Self {
        Quaternion::new(T::INFINITY, T::INFINITY, T::INFINITY, T::INFINITY)
    }
}

impl<T: NegInfinity> Quaternion<T> {
    /// Create a new [`Quaternion`] containing only negative infinities (-∞)
    pub const fn neg_infinity() -> Self {
        Quaternion::new(
            T::NEG_INFINITY,
            T::NEG_INFINITY,
            T::NEG_INFINITY,
            T::NEG_INFINITY,
        )
    }
}

impl<T: NaN> Quaternion<T> {
    /// Create a new [`Quaternion`] containing only `NaN` values
    pub const fn nan() -> Self {
        Quaternion::new(T::NAN, T::NAN, T::NAN, T::NAN)
    }
}

impl<
    T: Add<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + DivAssign
        + Sin
        + Cos
        + Sqrt
        + One
        + Clone
        + PartialEq,
> Quaternion<T>
{
    /// Produce a [`Quaternion`] representing a rotate of `angle` about `axis`
    pub fn angle_axis(angle: T, axis: Vector3<T>) -> Self {
        let axis = axis.normalized();
        let angle = angle / (T::ONE + T::ONE);
        Quaternion::new(
            axis.x * angle.clone().sin(),
            axis.y * angle.clone().sin(),
            axis.z * angle.clone().sin(),
            angle.cos(),
        )
    }
}

impl<
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Sin + Cos + One + Clone,
> Quaternion<T>
{
    /// Produce a [`Quaternion`] from `euler` angles
    pub fn euler(euler: Vector3<T>) -> Self {
        let roll = euler.x / (T::ONE + T::ONE);
        let pitch = euler.y / (T::ONE + T::ONE);
        let yaw = euler.z / (T::ONE + T::ONE);

        let cr = roll.clone().cos();
        let sr = roll.sin();
        let cp = pitch.clone().cos();
        let sp = pitch.sin();
        let cy = yaw.clone().cos();
        let sy = yaw.sin();

        Quaternion::new(
            sr.clone() * cp.clone() * cy.clone() - cr.clone() * sp.clone() * sy.clone(),
            cr.clone() * sp.clone() * cy.clone() + sr.clone() * cp.clone() * sy.clone(),
            cr.clone() * cp.clone() * sy.clone() - sr.clone() * sp.clone() * cy.clone(),
            cr * cp * cy + sr * sp * sy,
        )
    }
}
