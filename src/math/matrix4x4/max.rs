use crate::math::{Matrix4x4, number::Max};

impl<T: Max> Matrix4x4<T> {
    /// Compares and returns the maximum of two matrices component-wise
    pub fn max_m(self, max: Matrix4x4<T>) -> Matrix4x4<T> {
        let [
            [v00, v01, v02, v03],
            [v10, v11, v12, v13],
            [v20, v21, v22, v23],
            [v30, v31, v32, v33],
        ] = self.v;

        let [
            [max00, max01, max02, max03],
            [max10, max11, max12, max13],
            [max20, max21, max22, max23],
            [max30, max31, max32, max33],
        ] = max.v;

        Matrix4x4::new_row([
            [
                v00.max(max00),
                v01.max(max01),
                v02.max(max02),
                v03.max(max03),
            ],
            [
                v10.max(max10),
                v11.max(max11),
                v12.max(max12),
                v13.max(max13),
            ],
            [
                v20.max(max20),
                v21.max(max21),
                v22.max(max22),
                v23.max(max23),
            ],
            [
                v30.max(max30),
                v31.max(max31),
                v32.max(max32),
                v33.max(max33),
            ],
        ])
    }
}

impl<T: Max + Clone> Matrix4x4<T> {
    /// Compares and returns the maximum of a matrix component-wise and a scalar
    pub fn max(self, max: T) -> Matrix4x4<T> {
        let [
            [v00, v01, v02, v03],
            [v10, v11, v12, v13],
            [v20, v21, v22, v23],
            [v30, v31, v32, v33],
        ] = self.v;

        Matrix4x4::new_row([
            [
                v00.max(max.clone()),
                v01.max(max.clone()),
                v02.max(max.clone()),
                v03.max(max.clone()),
            ],
            [
                v10.max(max.clone()),
                v11.max(max.clone()),
                v12.max(max.clone()),
                v13.max(max.clone()),
            ],
            [
                v20.max(max.clone()),
                v21.max(max.clone()),
                v22.max(max.clone()),
                v23.max(max.clone()),
            ],
            [
                v30.max(max.clone()),
                v31.max(max.clone()),
                v32.max(max.clone()),
                v33.max(max),
            ],
        ])
    }
}

impl<T: Max> Max for Matrix4x4<T> {
    fn max(self, other: Self) -> Self {
        self.max_m(other)
    }
}
