use crate::math::{
    Matrix4x4, Quaternion, Vector3, Vector4,
    number::{Cos, Infinity, NaN, NegInfinity, One, Sin, Sqrt, Tan, Zero},
};
use std::ops::{Add, Div, DivAssign, Mul, Neg, Sub};

impl<T> Matrix4x4<T> {
    /// Create a new [`Matrix4x4`] in row-major order
    pub const fn new_row(v: [[T; 4]; 4]) -> Self {
        Matrix4x4 { v }
    }

    /// Create a new [`Matrix4x4`] in column-major order
    pub fn new_col(
        [
            [v00, v10, v20, v30],
            [v01, v11, v21, v31],
            [v02, v12, v22, v32],
            [v03, v13, v23, v33],
        ]: [[T; 4]; 4],
    ) -> Self {
        Matrix4x4::from_col_array([
            v00, v10, v20, v30, v01, v11, v21, v31, v02, v12, v22, v32, v03, v13, v23, v33,
        ])
    }

    /// Create a new [`Matrix4x4`] from an array in row-major order
    pub fn from_row_array(
        [
            v00,
            v01,
            v02,
            v03,
            v10,
            v11,
            v12,
            v13,
            v20,
            v21,
            v22,
            v23,
            v30,
            v31,
            v32,
            v33,
        ]: [T; 16],
    ) -> Self {
        Matrix4x4::new_row([
            [v00, v01, v02, v03],
            [v10, v11, v12, v13],
            [v20, v21, v22, v23],
            [v30, v31, v32, v33],
        ])
    }

    /// Create a new [`Matrix4x4`] from an array in column-major order
    pub fn from_col_array(
        [
            v00,
            v10,
            v20,
            v30,
            v01,
            v11,
            v21,
            v31,
            v02,
            v12,
            v22,
            v32,
            v03,
            v13,
            v23,
            v33,
        ]: [T; 16],
    ) -> Self {
        Matrix4x4::new_row([
            [v00, v01, v02, v03],
            [v10, v11, v12, v13],
            [v20, v21, v22, v23],
            [v30, v31, v32, v33],
        ])
    }

    /// Create a new [`Matrix4x4`] from an array of [`Vector4`]s in row-major order
    pub fn from_row_vec_array([v0, v1, v2, v3]: [Vector4<T>; 4]) -> Self {
        Matrix4x4::from_row_array([
            v0.x, v0.y, v0.z, v0.w, v1.x, v1.y, v1.z, v1.w, v2.x, v2.y, v2.z, v2.w, v3.x, v3.y,
            v3.z, v3.w,
        ])
    }

    /// Create a new [`Matrix4x4`] from an array of [`Vector4`]s in column-major order
    pub fn from_col_vec_array([v0, v1, v2, v3]: [Vector4<T>; 4]) -> Self {
        Matrix4x4::from_col_array([
            v0.x, v0.y, v0.z, v0.w, v1.x, v1.y, v1.z, v1.w, v2.x, v2.y, v2.z, v2.w, v3.x, v3.y,
            v3.z, v3.w,
        ])
    }
}

impl<T: Clone> Matrix4x4<T> {
    /// Create a new [`Matrix4x4`] from a slice in row-major order
    pub fn from_row_slice(s: &[T]) -> Self {
        assert!(s.len() >= 16);
        Matrix4x4::from_row_array([
            s[0].clone(),
            s[1].clone(),
            s[2].clone(),
            s[3].clone(),
            s[4].clone(),
            s[5].clone(),
            s[6].clone(),
            s[7].clone(),
            s[8].clone(),
            s[9].clone(),
            s[10].clone(),
            s[11].clone(),
            s[12].clone(),
            s[13].clone(),
            s[14].clone(),
            s[15].clone(),
        ])
    }

    /// Create a new [`Matrix4x4`] from a slice in column-major order
    pub fn from_col_slice(s: &[T]) -> Self {
        assert!(s.len() >= 16);
        Matrix4x4::from_col_array([
            s[0].clone(),
            s[1].clone(),
            s[2].clone(),
            s[3].clone(),
            s[4].clone(),
            s[5].clone(),
            s[6].clone(),
            s[7].clone(),
            s[8].clone(),
            s[9].clone(),
            s[10].clone(),
            s[11].clone(),
            s[12].clone(),
            s[13].clone(),
            s[14].clone(),
            s[15].clone(),
        ])
    }

    /// Create a new [`Matrix4x4`] from a slice of slices in row-major order
    pub fn from_row_slices(s: &[&[T]]) -> Self {
        assert!(s.len() >= 4);
        for i in 0..4 {
            assert!(s[i].len() >= 4);
        }

        Matrix4x4::from_row_array([
            s[0][0].clone(),
            s[0][1].clone(),
            s[0][2].clone(),
            s[0][3].clone(),
            s[1][0].clone(),
            s[1][1].clone(),
            s[1][2].clone(),
            s[1][3].clone(),
            s[2][0].clone(),
            s[2][1].clone(),
            s[2][2].clone(),
            s[2][3].clone(),
            s[3][0].clone(),
            s[3][1].clone(),
            s[3][2].clone(),
            s[3][3].clone(),
        ])
    }

    /// Create a new [`Matrix4x4`] from a slice of slices in column-major order
    pub fn from_col_slices(s: &[&[T]]) -> Self {
        assert!(s.len() >= 4);
        for i in 0..4 {
            assert!(s[i].len() >= 4);
        }

        Matrix4x4::from_col_array([
            s[0][0].clone(),
            s[0][1].clone(),
            s[0][2].clone(),
            s[0][3].clone(),
            s[1][0].clone(),
            s[1][1].clone(),
            s[1][2].clone(),
            s[1][3].clone(),
            s[2][0].clone(),
            s[2][1].clone(),
            s[2][2].clone(),
            s[2][3].clone(),
            s[3][0].clone(),
            s[3][1].clone(),
            s[3][2].clone(),
            s[3][3].clone(),
        ])
    }

    /// Create a new [`Matrix4x4`] from an array of [`Vector4`]s in row-major order
    pub fn from_row_vec_slice(v: &[Vector4<T>]) -> Self {
        assert!(v.len() >= 4);
        Matrix4x4::from_row_vec_array([v[0].clone(), v[1].clone(), v[2].clone(), v[3].clone()])
    }

    /// Create a new [`Matrix4x4`] from an array of [`Vector4`]s in column-major order
    pub fn from_col_vec_slice(v: &[Vector4<T>]) -> Self {
        assert!(v.len() >= 4);
        Matrix4x4::from_col_vec_array([v[0].clone(), v[1].clone(), v[2].clone(), v[3].clone()])
    }

    /// Create a new [`Matrix4x4`] consisting of the same values
    pub fn splat(v: T) -> Self {
        Matrix4x4::new_row([
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
        Matrix4x4::new_row([
            [T::ZERO, T::ZERO, T::ZERO, T::ZERO],
            [T::ZERO, T::ZERO, T::ZERO, T::ZERO],
            [T::ZERO, T::ZERO, T::ZERO, T::ZERO],
            [T::ZERO, T::ZERO, T::ZERO, T::ZERO],
        ])
    }

    /// Creates a new [`Matrix4x4`] with values along the diagonal, and nowhere else
    pub fn diagonal([d0, d1, d2, d3]: [T; 4]) -> Self {
        Matrix4x4::new_row([
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
        Matrix4x4::new_row([
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
        Matrix4x4::new_row([
            [T::ONE, T::ZERO, T::ZERO, T::ZERO],
            [T::ZERO, T::ONE, T::ZERO, T::ZERO],
            [T::ZERO, T::ZERO, T::ONE, T::ZERO],
            [T::ZERO, T::ZERO, T::ZERO, T::ONE],
        ])
    }

    /// Create a new [`Matrix4x4`] representation `translation`
    pub fn translation(translation: Vector3<T>) -> Self {
        Matrix4x4::new_row([
            [T::ONE, T::ZERO, T::ZERO, T::ZERO],
            [T::ZERO, T::ONE, T::ZERO, T::ZERO],
            [T::ZERO, T::ZERO, T::ONE, T::ZERO],
            [translation.x, translation.y, translation.z, T::ONE],
        ])
    }

    /// Create a new [`Matrix4x4`] representation `scale`
    pub fn scale(scale: Vector3<T>) -> Self {
        Matrix4x4::new_row([
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
        Matrix4x4::new_row([
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
        Matrix4x4::new_row([
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
        Matrix4x4::new_row([
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

        Matrix4x4::new_row([
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

        Matrix4x4::new_row([
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

        Matrix4x4::new_row([
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

impl<T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Neg<Output = T> + Zero + One + Clone>
    Matrix4x4<T>
{
    /// Create a new [`Matrix4x4`] equivalent to the rotation represented by `rot`
    pub fn rotation(rot: Quaternion<T>) -> Self {
        let two = T::ONE + T::ONE;

        let xx = rot.x.clone() * rot.x.clone();
        let yy = rot.y.clone() * rot.y.clone();
        let zz = rot.z.clone() * rot.z.clone();
        let xy = rot.x.clone() * rot.y.clone();
        let xz = rot.x.clone() * rot.z.clone();
        let yz = rot.y.clone() * rot.z.clone();
        let wx = rot.w.clone() * rot.x;
        let wy = rot.w.clone() * rot.y;
        let wz = rot.w * rot.z;

        Matrix4x4::new_row([
            [
                T::ONE - two.clone() * (yy.clone() + zz.clone()),
                two.clone() * (xy.clone() - wz.clone()),
                two.clone() * (xz.clone() + wy.clone()),
                T::ZERO,
            ],
            [
                two.clone() * (xy + wz),
                T::ONE - two.clone() * (xx.clone() + zz),
                two.clone() * (yz.clone() - wx.clone()),
                T::ZERO,
            ],
            [
                two.clone() * (xz - wy),
                two.clone() * (yz + wx),
                T::ONE - two * (xx + yy),
                T::ZERO,
            ],
            [T::ZERO, T::ZERO, T::ZERO, T::ONE],
        ])
    }
}

impl<
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Neg<Output = T>
        + DivAssign
        + Clone
        + Sqrt
        + PartialEq
        + Zero
        + One,
> Matrix4x4<T>
{
    /// Create a matrix which has an object at `position` looking at `target`
    pub fn look_at(position: Vector3<T>, target: Vector3<T>, up: Vector3<T>) -> Self {
        let forward = (target - position.clone()).normalized();
        let right = up.cross(forward.clone()).normalized();
        let up = forward.clone().cross(right.clone());

        let right2 = right.clone();
        let up2 = up.clone();
        let forward2 = forward.clone();
        Matrix4x4::new_row([
            [right.x, up.x, forward.x, T::ZERO],
            [right.y, up.y, forward.y, T::ZERO],
            [right.z, up.z, forward.z, T::ZERO],
            [
                -right2.dot(position.clone()),
                -up2.dot(position.clone()),
                -forward2.dot(position),
                T::ONE,
            ],
        ])
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
    /// Create a left-hand perspective projection matrix
    pub fn perspective_lh(aspect: T, fov: T, near: T, far: T) -> Self {
        let y_scale = T::ONE / (fov / (T::ONE + T::ONE)).tan();
        let x_scale = y_scale.clone() / aspect;
        Matrix4x4::new_row([
            [x_scale, T::ZERO, T::ZERO, T::ZERO],
            [T::ZERO, y_scale, T::ZERO, T::ZERO],
            [
                T::ZERO,
                T::ZERO,
                (far.clone()) / (far.clone() - near.clone()),
                T::ONE,
            ],
            [
                T::ZERO,
                T::ZERO,
                (far.clone() * -near.clone()) / (far - near),
                T::ZERO,
            ],
        ])
    }
}

impl<T: Zero + One + Add<Output = T> + Sub<Output = T> + Div<Output = T> + Neg<Output = T> + Clone>
    Matrix4x4<T>
{
    /// Creates an orthographics projection matrix
    pub fn orthographic(left: T, right: T, top: T, bottom: T, near: T, far: T) -> Self {
        Matrix4x4::new_row([
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
