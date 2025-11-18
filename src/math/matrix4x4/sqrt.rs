use crate::math::{Matrix4x4, number::Sqrt};

impl<T: Sqrt> Matrix4x4<T> {
    /// Gets the component-wise square root
    pub fn sqrt(self) -> Self {
        let [
            [v00, v01, v02, v03],
            [v10, v11, v12, v13],
            [v20, v21, v22, v23],
            [v30, v31, v32, v33],
        ] = self.v;

        Matrix4x4::new_row([
            [v00.sqrt(), v01.sqrt(), v02.sqrt(), v03.sqrt()],
            [v10.sqrt(), v11.sqrt(), v12.sqrt(), v13.sqrt()],
            [v20.sqrt(), v21.sqrt(), v22.sqrt(), v23.sqrt()],
            [v30.sqrt(), v31.sqrt(), v32.sqrt(), v33.sqrt()],
        ])
    }
}

impl<T: Sqrt> Sqrt for Matrix4x4<T> {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
