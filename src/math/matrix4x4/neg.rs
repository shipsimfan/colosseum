use crate::math::Matrix4x4;
use std::ops::Neg;

impl<T: Neg<Output = T>> Neg for Matrix4x4<T> {
    type Output = Matrix4x4<T>;

    fn neg(self) -> Self::Output {
        let [
            [v00, v01, v02, v03],
            [v10, v11, v12, v13],
            [v20, v21, v22, v23],
            [v30, v31, v32, v33],
        ] = self.v;

        Matrix4x4::new_row([
            [-v00, -v01, -v02, -v03],
            [-v10, -v11, -v12, -v13],
            [-v20, -v21, -v22, -v23],
            [-v30, -v31, -v32, -v33],
        ])
    }
}
