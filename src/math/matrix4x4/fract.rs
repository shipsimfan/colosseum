use crate::math::{Matrix4x4, number::Fract};

impl<T: Fract> Matrix4x4<T> {
    /// Gets the fractional parts of the values of a [`Matrix4x4`] component-wise
    pub fn fract(self) -> Self {
        let [
            [v00, v01, v02, v03],
            [v10, v11, v12, v13],
            [v20, v21, v22, v23],
            [v30, v31, v32, v33],
        ] = self.v;

        Matrix4x4::new_row([
            [v00.fract(), v01.fract(), v02.fract(), v03.fract()],
            [v10.fract(), v11.fract(), v12.fract(), v13.fract()],
            [v20.fract(), v21.fract(), v22.fract(), v23.fract()],
            [v30.fract(), v31.fract(), v32.fract(), v33.fract()],
        ])
    }
}

impl<T: Fract> Fract for Matrix4x4<T> {
    fn fract(self) -> Self {
        self.fract()
    }
}
