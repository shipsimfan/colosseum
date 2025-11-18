use crate::math::{
    Matrix4x4,
    number::{Max, Min},
};

impl<T: Min + Max> Matrix4x4<T> {
    /// Clamps the values of the matrix between two matrices component-wise
    pub fn clamp_m(self, min: Matrix4x4<T>, max: Matrix4x4<T>) -> Matrix4x4<T> {
        let [
            [v00, v01, v02, v03],
            [v10, v11, v12, v13],
            [v20, v21, v22, v23],
            [v30, v31, v32, v33],
        ] = self.v;

        let [
            [min00, min01, min02, min03],
            [min10, min11, min12, min13],
            [min20, min21, min22, min23],
            [min30, min31, min32, min33],
        ] = min.v;

        let [
            [max00, max01, max02, max03],
            [max10, max11, max12, max13],
            [max20, max21, max22, max23],
            [max30, max31, max32, max33],
        ] = max.v;

        Matrix4x4::new_row([
            [
                v00.max(min00).min(max00),
                v01.max(min01).min(max01),
                v02.max(min02).min(max02),
                v03.max(min03).min(max03),
            ],
            [
                v10.max(min10).min(max10),
                v11.max(min11).min(max11),
                v12.max(min12).min(max12),
                v13.max(min13).min(max13),
            ],
            [
                v20.max(min20).min(max20),
                v21.max(min21).min(max21),
                v22.max(min22).min(max22),
                v23.max(min23).min(max23),
            ],
            [
                v30.max(min30).min(max30),
                v31.max(min31).min(max31),
                v32.max(min32).min(max32),
                v33.max(min33).min(max33),
            ],
        ])
    }
}

impl<T: Min + Max + Clone> Matrix4x4<T> {
    /// Clamps the values of the matrix between two values component-wise
    pub fn clamp(self, min: T, max: T) -> Matrix4x4<T> {
        let [
            [v00, v01, v02, v03],
            [v10, v11, v12, v13],
            [v20, v21, v22, v23],
            [v30, v31, v32, v33],
        ] = self.v;

        Matrix4x4::new_row([
            [
                v00.max(min.clone()).min(max.clone()),
                v01.max(min.clone()).min(max.clone()),
                v02.max(min.clone()).min(max.clone()),
                v03.max(min.clone()).min(max.clone()),
            ],
            [
                v10.max(min.clone()).min(max.clone()),
                v11.max(min.clone()).min(max.clone()),
                v12.max(min.clone()).min(max.clone()),
                v13.max(min.clone()).min(max.clone()),
            ],
            [
                v20.max(min.clone()).min(max.clone()),
                v21.max(min.clone()).min(max.clone()),
                v22.max(min.clone()).min(max.clone()),
                v23.max(min.clone()).min(max.clone()),
            ],
            [
                v30.max(min.clone()).min(max.clone()),
                v31.max(min.clone()).min(max.clone()),
                v32.max(min.clone()).min(max.clone()),
                v33.max(min).min(max),
            ],
        ])
    }
}
