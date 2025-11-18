use crate::math::{Matrix4x4, Matrix4x4f, number::IntoF32};

impl<T: IntoF32> Matrix4x4<T> {
    /// Convert this [`Matrix4x4<T>`] into a [`Matrix4x4f`]
    pub fn into_f32(self) -> Matrix4x4f {
        let [
            [v00, v01, v02, v03],
            [v10, v11, v12, v13],
            [v20, v21, v22, v23],
            [v30, v31, v32, v33],
        ] = self.v;

        Matrix4x4::new_row([
            [
                v00.into_f32(),
                v01.into_f32(),
                v02.into_f32(),
                v03.into_f32(),
            ],
            [
                v10.into_f32(),
                v11.into_f32(),
                v12.into_f32(),
                v13.into_f32(),
            ],
            [
                v20.into_f32(),
                v21.into_f32(),
                v22.into_f32(),
                v23.into_f32(),
            ],
            [
                v30.into_f32(),
                v31.into_f32(),
                v32.into_f32(),
                v33.into_f32(),
            ],
        ])
    }
}
