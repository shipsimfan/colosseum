use crate::math::{Matrix4x4, number::Absolute};

impl<T: Absolute> Matrix4x4<T> {
    /// Gets the aboslute value of a [`Matrix4x4`] component wise
    pub fn abs(self) -> Self {
        let [
            [v00, v01, v02, v03],
            [v10, v11, v12, v13],
            [v20, v21, v22, v23],
            [v30, v31, v32, v33],
        ] = self.v;

        Matrix4x4::new_row([
            [v00.abs(), v01.abs(), v02.abs(), v03.abs()],
            [v10.abs(), v11.abs(), v12.abs(), v13.abs()],
            [v20.abs(), v21.abs(), v22.abs(), v23.abs()],
            [v30.abs(), v31.abs(), v32.abs(), v33.abs()],
        ])
    }
}

impl<T: Absolute> Absolute for Matrix4x4<T> {
    fn abs(self) -> Self {
        self.abs()
    }
}
