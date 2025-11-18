use crate::math::{Matrix4x4, number::Floor};

impl<T: Floor> Matrix4x4<T> {
    /// Rounds the values of [`Matrix4x4`] component wise down to the nearest integer
    pub fn floor(self) -> Self {
        let [
            [v00, v01, v02, v03],
            [v10, v11, v12, v13],
            [v20, v21, v22, v23],
            [v30, v31, v32, v33],
        ] = self.v;

        Matrix4x4::new_row([
            [v00.floor(), v01.floor(), v02.floor(), v03.floor()],
            [v10.floor(), v11.floor(), v12.floor(), v13.floor()],
            [v20.floor(), v21.floor(), v22.floor(), v23.floor()],
            [v30.floor(), v31.floor(), v32.floor(), v33.floor()],
        ])
    }
}

impl<T: Floor> Floor for Matrix4x4<T> {
    fn floor(self) -> Self {
        self.floor()
    }
}
