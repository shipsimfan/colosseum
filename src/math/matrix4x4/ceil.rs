use crate::math::{Matrix4x4, number::Ceil};

impl<T: Ceil> Matrix4x4<T> {
    /// Rounds the values of [`Matrix4x4`] component wise up to the nearest integer
    pub fn ceil(self) -> Self {
        let [
            [v00, v01, v02, v03],
            [v10, v11, v12, v13],
            [v20, v21, v22, v23],
            [v30, v31, v32, v33],
        ] = self.v;

        Matrix4x4::new_row([
            [v00.ceil(), v01.ceil(), v02.ceil(), v03.ceil()],
            [v10.ceil(), v11.ceil(), v12.ceil(), v13.ceil()],
            [v20.ceil(), v21.ceil(), v22.ceil(), v23.ceil()],
            [v30.ceil(), v31.ceil(), v32.ceil(), v33.ceil()],
        ])
    }
}

impl<T: Ceil> Ceil for Matrix4x4<T> {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
