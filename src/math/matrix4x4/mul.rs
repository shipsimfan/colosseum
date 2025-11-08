use crate::math::{Matrix4x4, Vector4};
use std::ops::{Add, Mul};

impl<T: Mul<Output = T> + Clone> Mul<T> for Matrix4x4<T> {
    type Output = Matrix4x4<T>;

    fn mul(self, rhs: T) -> Self::Output {
        let [[v00, v01, v02, v03], [v10, v11, v12, v13], [v20, v21, v22, v23], [v30, v31, v32, v33]] =
            self.v;

        Matrix4x4::new([
            [
                v00 * rhs.clone(),
                v01 * rhs.clone(),
                v02 * rhs.clone(),
                v03 * rhs.clone(),
            ],
            [
                v10 * rhs.clone(),
                v11 * rhs.clone(),
                v12 * rhs.clone(),
                v13 * rhs.clone(),
            ],
            [
                v20 * rhs.clone(),
                v21 * rhs.clone(),
                v22 * rhs.clone(),
                v23 * rhs.clone(),
            ],
            [v30 * rhs.clone(), v31 * rhs.clone(), v32 * rhs.clone(), v33],
        ])
    }
}

impl<T: Mul<Output = T> + Add<Output = T> + Clone> Mul<Matrix4x4<T>> for Vector4<T> {
    type Output = Vector4<T>;

    fn mul(self, rhs: Matrix4x4<T>) -> Self::Output {
        let [[v00, v01, v02, v03], [v10, v11, v12, v13], [v20, v21, v22, v23], [v30, v31, v32, v33]] =
            rhs.v;

        Vector4::new(
            v00 * self.x.clone()
                + v01 * self.y.clone()
                + v02 * self.z.clone()
                + v03 * self.w.clone(),
            v10 * self.x.clone()
                + v11 * self.y.clone()
                + v12 * self.z.clone()
                + v13 * self.w.clone(),
            v20 * self.x.clone()
                + v21 * self.y.clone()
                + v22 * self.z.clone()
                + v23 * self.w.clone(),
            v30 * self.x + v31 * self.y + v32 * self.z + v33 * self.w,
        )
    }
}

impl<T: Mul<Output = T> + Add<Output = T> + Clone> Mul<Vector4<T>> for Matrix4x4<T> {
    type Output = Vector4<T>;

    fn mul(self, rhs: Vector4<T>) -> Self::Output {
        let [[v00, v01, v02, v03], [v10, v11, v12, v13], [v20, v21, v22, v23], [v30, v31, v32, v33]] =
            self.v;

        Vector4::new(
            v00 * rhs.x.clone() + v10 * rhs.y.clone() + v20 * rhs.z.clone() + v30 * rhs.w.clone(),
            v01 * rhs.x.clone() + v11 * rhs.y.clone() + v21 * rhs.z.clone() + v31 * rhs.w.clone(),
            v02 * rhs.x.clone() + v12 * rhs.y.clone() + v22 * rhs.z.clone() + v32 * rhs.w.clone(),
            v03 * rhs.x + v13 * rhs.y + v23 * rhs.z + v33 * rhs.w,
        )
    }
}

impl<T: Mul<Output = T> + Add<Output = T> + Clone> Mul for Matrix4x4<T> {
    type Output = Matrix4x4<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        let [[b00, b01, b02, b03], [b10, b11, b12, b13], [b20, b21, b22, b23], [b30, b31, b32, b33]] =
            rhs.v;

        let [c00, c10, c20, c30] = (Vector4::new(b00, b10, b20, b30) * self.clone()).into();
        let [c01, c11, c21, c31] = (Vector4::new(b01, b11, b21, b31) * self.clone()).into();
        let [c02, c12, c22, c32] = (Vector4::new(b02, b12, b22, b32) * self.clone()).into();
        let [c03, c13, c23, c33] = (Vector4::new(b03, b13, b23, b33) * self).into();

        Matrix4x4::new([
            [c00, c01, c02, c03],
            [c10, c11, c12, c13],
            [c20, c21, c22, c23],
            [c30, c31, c32, c33],
        ])
    }
}

#[cfg(test)]
mod tests {
    #[test]
    pub fn simple_mat_mat_mul() {
        const A: crate::math::Matrix4x4u = crate::math::Matrix4x4u::new([
            [1, 2, 3, 4],
            [5, 6, 7, 8],
            [9, 10, 11, 12],
            [13, 14, 15, 16],
        ]);

        const B: crate::math::Matrix4x4u = crate::math::Matrix4x4u::new([
            [17, 18, 19, 20],
            [21, 22, 23, 24],
            [25, 26, 27, 28],
            [29, 30, 31, 32],
        ]);

        const TARGET: crate::math::Matrix4x4u = crate::math::Matrix4x4u::new([
            [250, 260, 270, 280],
            [618, 644, 670, 696],
            [986, 1028, 1070, 1112],
            [1354, 1412, 1470, 1528],
        ]);

        assert_eq!(A * B, TARGET);
    }
}
