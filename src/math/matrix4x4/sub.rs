use crate::math::Matrix4x4;
use std::ops::{Sub, SubAssign};

impl<T: Sub<Output = T> + Clone> Sub<T> for Matrix4x4<T> {
    type Output = Matrix4x4<T>;

    fn sub(self, rhs: T) -> Self::Output {
        let [
            [v00, v01, v02, v03],
            [v10, v11, v12, v13],
            [v20, v21, v22, v23],
            [v30, v31, v32, v33],
        ] = self.v;

        Matrix4x4::new_row([
            [
                v00 - rhs.clone(),
                v01 - rhs.clone(),
                v02 - rhs.clone(),
                v03 - rhs.clone(),
            ],
            [
                v10 - rhs.clone(),
                v11 - rhs.clone(),
                v12 - rhs.clone(),
                v13 - rhs.clone(),
            ],
            [
                v20 - rhs.clone(),
                v21 - rhs.clone(),
                v22 - rhs.clone(),
                v23 - rhs.clone(),
            ],
            [
                v30 - rhs.clone(),
                v31 - rhs.clone(),
                v32 - rhs.clone(),
                v33 - rhs,
            ],
        ])
    }
}

impl<T: Sub<Output = T>> Sub for Matrix4x4<T> {
    type Output = Matrix4x4<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        let [
            [a00, a01, a02, a03],
            [a10, a11, a12, a13],
            [a20, a21, a22, a23],
            [a30, a31, a32, a33],
        ] = self.v;

        let [
            [b00, b01, b02, b03],
            [b10, b11, b12, b13],
            [b20, b21, b22, b23],
            [b30, b31, b32, b33],
        ] = rhs.v;

        Matrix4x4::new_row([
            [a00 - b00, a01 - b01, a02 - b02, a03 - b03],
            [a10 - b10, a11 - b11, a12 - b12, a13 - b13],
            [a20 - b20, a21 - b21, a22 - b22, a23 - b23],
            [a30 - b30, a31 - b31, a32 - b32, a33 - b33],
        ])
    }
}

impl<T: SubAssign + Clone> SubAssign<T> for Matrix4x4<T> {
    fn sub_assign(&mut self, rhs: T) {
        self.v[0][0] -= rhs.clone();
        self.v[0][1] -= rhs.clone();
        self.v[0][2] -= rhs.clone();
        self.v[0][3] -= rhs.clone();
        self.v[1][0] -= rhs.clone();
        self.v[1][1] -= rhs.clone();
        self.v[1][2] -= rhs.clone();
        self.v[1][3] -= rhs.clone();
        self.v[2][0] -= rhs.clone();
        self.v[2][1] -= rhs.clone();
        self.v[2][2] -= rhs.clone();
        self.v[2][3] -= rhs.clone();
        self.v[3][0] -= rhs.clone();
        self.v[3][1] -= rhs.clone();
        self.v[3][2] -= rhs.clone();
        self.v[3][3] -= rhs;
    }
}

impl<T: SubAssign> SubAssign for Matrix4x4<T> {
    fn sub_assign(&mut self, rhs: Self) {
        let [
            [b00, b01, b02, b03],
            [b10, b11, b12, b13],
            [b20, b21, b22, b23],
            [b30, b31, b32, b33],
        ] = rhs.v;

        self.v[0][0] -= b00;
        self.v[0][1] -= b01;
        self.v[0][2] -= b02;
        self.v[0][3] -= b03;
        self.v[1][0] -= b10;
        self.v[1][1] -= b11;
        self.v[1][2] -= b12;
        self.v[1][3] -= b13;
        self.v[2][0] -= b20;
        self.v[2][1] -= b21;
        self.v[2][2] -= b22;
        self.v[2][3] -= b23;
        self.v[3][0] -= b30;
        self.v[3][1] -= b31;
        self.v[3][2] -= b32;
        self.v[3][3] -= b33;
    }
}
