use crate::math::{Matrix4x4, number::Round};

impl<T: Round> Matrix4x4<T> {
    /// Rounds the values of [`Matrix4x4`] component wise to the nearest integer
    pub fn round(self) -> Self {
        let [
            [v00, v01, v02, v03],
            [v10, v11, v12, v13],
            [v20, v21, v22, v23],
            [v30, v31, v32, v33],
        ] = self.v;

        Matrix4x4::new_row([
            [v00.round(), v01.round(), v02.round(), v03.round()],
            [v10.round(), v11.round(), v12.round(), v13.round()],
            [v20.round(), v21.round(), v22.round(), v23.round()],
            [v30.round(), v31.round(), v32.round(), v33.round()],
        ])
    }
}

impl<T: Round> Round for Matrix4x4<T> {
    fn round(self) -> Self {
        self.round()
    }
}
