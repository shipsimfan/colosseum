use crate::math::{
    number::{Cos, Infinity, NaN, NegInfinity, One, Sin, Tan, Zero},
    Matrix4x4, Vector3,
};
use std::ops::{Add, Div, Mul, Neg, Sub};

impl<T> Matrix4x4<T> {
    /// Create a new [`Matrix4x4`]
    pub const fn new(v: [[T; 4]; 4]) -> Self {
        Matrix4x4 { v }
    }

    /// Create a new [`Matrix4x4`] from an array
    pub fn from_array(
        [v00, v01, v02, v03, v10, v11, v12, v13, v20, v21, v22, v23, v30, v31, v32, v33]: [T; 16],
    ) -> Self {
        Matrix4x4::new([
            [v00, v01, v02, v03],
            [v10, v11, v12, v13],
            [v20, v21, v22, v23],
            [v30, v31, v32, v33],
        ])
    }
}

impl<T: Clone> Matrix4x4<T> {
    /// Create a new [`Matrix4x4`] from a slice
    pub fn from_slice(s: &[T]) -> Self {
        assert!(s.len() >= 16);
        Matrix4x4::new([
            [s[0].clone(), s[1].clone(), s[2].clone(), s[3].clone()],
            [s[4].clone(), s[5].clone(), s[6].clone(), s[7].clone()],
            [s[8].clone(), s[9].clone(), s[10].clone(), s[11].clone()],
            [s[12].clone(), s[13].clone(), s[14].clone(), s[15].clone()],
        ])
    }

    /// Create a new [`Matrix4x4`] from a slice of slices
    pub fn from_slices(s: &[&[T]]) -> Self {
        assert!(s.len() >= 4);
        for i in 0..4 {
            assert!(s[i].len() >= 4);
        }

        Matrix4x4::new([
            [
                s[0][0].clone(),
                s[0][1].clone(),
                s[0][2].clone(),
                s[0][3].clone(),
            ],
            [
                s[1][0].clone(),
                s[1][1].clone(),
                s[1][2].clone(),
                s[1][3].clone(),
            ],
            [
                s[2][0].clone(),
                s[2][1].clone(),
                s[2][2].clone(),
                s[2][3].clone(),
            ],
            [
                s[3][0].clone(),
                s[3][1].clone(),
                s[3][2].clone(),
                s[3][3].clone(),
            ],
        ])
    }

    /// Create a new [`Matrix4x4`] consisting of the same values
    pub fn splat(v: T) -> Self {
        Matrix4x4::new([
            [v.clone(), v.clone(), v.clone(), v.clone()],
            [v.clone(), v.clone(), v.clone(), v.clone()],
            [v.clone(), v.clone(), v.clone(), v.clone()],
            [v.clone(), v.clone(), v.clone(), v],
        ])
    }
}

impl<T: Zero> Matrix4x4<T> {
    /// Create a new [`Matrix4x4`] containing only zeroes
    pub const fn zero() -> Self {
        Matrix4x4::new([
            [T::ZERO, T::ZERO, T::ZERO, T::ZERO],
            [T::ZERO, T::ZERO, T::ZERO, T::ZERO],
            [T::ZERO, T::ZERO, T::ZERO, T::ZERO],
            [T::ZERO, T::ZERO, T::ZERO, T::ZERO],
        ])
    }

    /// Creates a new [`Matrix4x4`] with values along the diagonal, and nowhere else
    pub fn diagonal([d0, d1, d2, d3]: [T; 4]) -> Self {
        Matrix4x4::new([
            [d0, T::ZERO, T::ZERO, T::ZERO],
            [T::ZERO, d1, T::ZERO, T::ZERO],
            [T::ZERO, T::ZERO, d2, T::ZERO],
            [T::ZERO, T::ZERO, T::ZERO, d3],
        ])
    }
}

impl<T: One> Matrix4x4<T> {
    /// Create a new [`Matrix4x4`] containing only ones
    pub const fn one() -> Self {
        Matrix4x4::new([
            [T::ONE, T::ONE, T::ONE, T::ONE],
            [T::ONE, T::ONE, T::ONE, T::ONE],
            [T::ONE, T::ONE, T::ONE, T::ONE],
            [T::ONE, T::ONE, T::ONE, T::ONE],
        ])
    }
}

impl<T: Zero + One> Matrix4x4<T> {
    /// Create a new identity [`Matrix4x4`]
    pub const fn identity() -> Self {
        Matrix4x4::new([
            [T::ONE, T::ZERO, T::ZERO, T::ZERO],
            [T::ZERO, T::ONE, T::ZERO, T::ZERO],
            [T::ZERO, T::ZERO, T::ONE, T::ZERO],
            [T::ZERO, T::ZERO, T::ZERO, T::ONE],
        ])
    }

    /// Create a new [`Matrix4x4`] representation `translation`
    pub fn translation(translation: Vector3<T>) -> Self {
        Matrix4x4::new([
            [T::ONE, T::ZERO, T::ZERO, T::ZERO],
            [T::ZERO, T::ONE, T::ZERO, T::ZERO],
            [T::ZERO, T::ZERO, T::ONE, T::ZERO],
            [translation.x, translation.y, translation.z, T::ONE],
        ])
    }

    /// Create a new [`Matrix4x4`] representation `scale`
    pub fn scale(scale: Vector3<T>) -> Self {
        Matrix4x4::new([
            [scale.x, T::ZERO, T::ZERO, T::ZERO],
            [T::ZERO, scale.y, T::ZERO, T::ZERO],
            [T::ZERO, T::ZERO, scale.z, T::ZERO],
            [T::ZERO, T::ZERO, T::ZERO, T::ONE],
        ])
    }
}

impl<T: Infinity> Matrix4x4<T> {
    /// Create a new [`Matrix4x4`] containing only infinities (∞)
    pub const fn infinity() -> Self {
        Matrix4x4::new([
            [T::INFINITY, T::INFINITY, T::INFINITY, T::INFINITY],
            [T::INFINITY, T::INFINITY, T::INFINITY, T::INFINITY],
            [T::INFINITY, T::INFINITY, T::INFINITY, T::INFINITY],
            [T::INFINITY, T::INFINITY, T::INFINITY, T::INFINITY],
        ])
    }
}

impl<T: NegInfinity> Matrix4x4<T> {
    /// Create a new [`Matrix4x4`] containing only negative infinities (-∞)
    pub const fn neg_infinity() -> Self {
        Matrix4x4::new([
            [
                T::NEG_INFINITY,
                T::NEG_INFINITY,
                T::NEG_INFINITY,
                T::NEG_INFINITY,
            ],
            [
                T::NEG_INFINITY,
                T::NEG_INFINITY,
                T::NEG_INFINITY,
                T::NEG_INFINITY,
            ],
            [
                T::NEG_INFINITY,
                T::NEG_INFINITY,
                T::NEG_INFINITY,
                T::NEG_INFINITY,
            ],
            [
                T::NEG_INFINITY,
                T::NEG_INFINITY,
                T::NEG_INFINITY,
                T::NEG_INFINITY,
            ],
        ])
    }
}

impl<T: NaN> Matrix4x4<T> {
    /// Create a new [`Matrix4x4`] containing only `NaN` values
    pub const fn nan() -> Self {
        Matrix4x4::new([
            [T::NAN, T::NAN, T::NAN, T::NAN],
            [T::NAN, T::NAN, T::NAN, T::NAN],
            [T::NAN, T::NAN, T::NAN, T::NAN],
            [T::NAN, T::NAN, T::NAN, T::NAN],
        ])
    }
}

impl<T: Zero + One + Cos + Sin + Clone + Neg<Output = T>> Matrix4x4<T> {
    /// Create a new [`Matrix4x4`] for an `angle` radian rotation about the x-axis
    pub fn x_rotation(angle: T) -> Self {
        let c = angle.clone().cos();
        let s = angle.sin();

        Matrix4x4::new([
            [T::ONE, T::ZERO, T::ZERO, T::ZERO],
            [T::ZERO, c.clone(), s.clone(), T::ZERO],
            [T::ZERO, -s, c, T::ZERO],
            [T::ZERO, T::ZERO, T::ZERO, T::ONE],
        ])
    }

    /// Create a new [`Matrix4x4`] for an `angle` radian rotation about the y-axis
    pub fn y_rotation(angle: T) -> Self {
        let c = angle.clone().cos();
        let s = angle.sin();

        Matrix4x4::new([
            [c.clone(), T::ZERO, -s.clone(), T::ZERO],
            [T::ZERO, T::ONE, T::ZERO, T::ZERO],
            [s, T::ZERO, c, T::ZERO],
            [T::ZERO, T::ZERO, T::ZERO, T::ONE],
        ])
    }

    /// Create a new [`Matrix4x4`] for an `angle` radian rotation about the z-axis
    pub fn z_rotation(angle: T) -> Self {
        let c = angle.clone().cos();
        let s = angle.sin();

        Matrix4x4::new([
            [c.clone(), s.clone(), T::ZERO, T::ZERO],
            [-s, c, T::ZERO, T::ZERO],
            [T::ZERO, T::ZERO, T::ONE, T::ZERO],
            [T::ZERO, T::ZERO, T::ZERO, T::ONE],
        ])
    }
}

impl<T: Zero + One + Cos + Sin + Clone + Neg<Output = T> + Add<Output = T> + Mul<Output = T>>
    Matrix4x4<T>
{
    /// Create a new [`Matrix4x4`] for radian `angles` rotation about the x, y, and z axess
    pub fn euler_rotation(angles: Vector3<T>) -> Self {
        Matrix4x4::z_rotation(angles.z)
            * Matrix4x4::y_rotation(angles.y)
            * Matrix4x4::x_rotation(angles.x)
    }
}

impl<
        T: Zero
            + One
            + Div<Output = T>
            + Mul<Output = T>
            + Add<Output = T>
            + Sub<Output = T>
            + Neg<Output = T>
            + Tan
            + Clone,
    > Matrix4x4<T>
{
    /// Create a perspective projection matrix
    pub fn perspective(aspect: T, fov: T, near: T, far: T) -> Self {
        Matrix4x4::new([
            [
                T::ONE / (aspect * (fov.clone() / (T::ONE + T::ONE)).tan()),
                T::ZERO,
                T::ZERO,
                T::ZERO,
            ],
            [
                T::ZERO,
                T::ONE / (fov / (T::ONE + T::ONE)).tan(),
                T::ZERO,
                T::ZERO,
            ],
            [
                T::ZERO,
                T::ZERO,
                (far.clone() + near.clone()) / (far.clone() - near.clone()),
                T::ONE,
            ],
            [
                T::ZERO,
                T::ZERO,
                -(((T::ONE + T::ONE) * far.clone() * near.clone()) / (far - near)),
                T::ZERO,
            ],
        ])
    }
}

impl<
        T: Zero + One + Add<Output = T> + Sub<Output = T> + Div<Output = T> + Neg<Output = T> + Clone,
    > Matrix4x4<T>
{
    /// Creates an orthographics projection matrix
    pub fn orthographic(left: T, right: T, top: T, bottom: T, near: T, far: T) -> Self {
        Matrix4x4::new([
            [
                (T::ONE + T::ONE) / (right.clone() - left.clone()),
                T::ZERO,
                T::ZERO,
                -((right.clone() + left.clone()) / (right - left)),
            ],
            [
                T::ZERO,
                (T::ONE + T::ONE) / (top.clone() - bottom.clone()),
                T::ZERO,
                -((top.clone() + bottom.clone()) / (top - bottom)),
            ],
            [
                T::ZERO,
                T::ZERO,
                -(T::ONE + T::ONE) / (far.clone() - near.clone()),
                -((far.clone() + near.clone()) / (far - near)),
            ],
            [T::ZERO, T::ZERO, T::ZERO, T::ONE],
        ])
    }
}
