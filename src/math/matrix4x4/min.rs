use crate::math::{Matrix4x4, number::Min};

impl<T: Min> Matrix4x4<T> {
    /// Compares and returns the minimum of two matrices component-wise
    pub fn min_m(self, min: Matrix4x4<T>) -> Matrix4x4<T> {
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

        Matrix4x4::new_row([
            [
                v00.min(min00),
                v01.min(min01),
                v02.min(min02),
                v03.min(min03),
            ],
            [
                v10.min(min10),
                v11.min(min11),
                v12.min(min12),
                v13.min(min13),
            ],
            [
                v20.min(min20),
                v21.min(min21),
                v22.min(min22),
                v23.min(min23),
            ],
            [
                v30.min(min30),
                v31.min(min31),
                v32.min(min32),
                v33.min(min33),
            ],
        ])
    }
}

impl<T: Min + Clone> Matrix4x4<T> {
    /// Compares and returns the minimum of a matrix component-wise and a scalar
    pub fn min(self, min: T) -> Matrix4x4<T> {
        let [
            [v00, v01, v02, v03],
            [v10, v11, v12, v13],
            [v20, v21, v22, v23],
            [v30, v31, v32, v33],
        ] = self.v;

        Matrix4x4::new_row([
            [
                v00.min(min.clone()),
                v01.min(min.clone()),
                v02.min(min.clone()),
                v03.min(min.clone()),
            ],
            [
                v10.min(min.clone()),
                v11.min(min.clone()),
                v12.min(min.clone()),
                v13.min(min.clone()),
            ],
            [
                v20.min(min.clone()),
                v21.min(min.clone()),
                v22.min(min.clone()),
                v23.min(min.clone()),
            ],
            [
                v30.min(min.clone()),
                v31.min(min.clone()),
                v32.min(min.clone()),
                v33.min(min),
            ],
        ])
    }
}

impl<T: Min> Min for Matrix4x4<T> {
    fn min(self, other: Self) -> Self {
        self.min_m(other)
    }
}
