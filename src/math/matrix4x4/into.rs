use crate::math::{Matrix4x4, Vector4};

impl<T> Matrix4x4<T> {
    /// Convert this [`Matrix4x4`] into an array of arrays in row-major ordering
    pub fn into_row_arrays(self) -> [[T; 4]; 4] {
        self.v
    }

    /// Convert this [`Matrix4x4`] into an array of arrays in col-major ordering
    pub fn into_col_arrays(self) -> [[T; 4]; 4] {
        let [
            [v00, v01, v02, v03],
            [v10, v11, v12, v13],
            [v20, v21, v22, v23],
            [v30, v31, v32, v33],
        ] = self.v;

        [
            [v00, v10, v20, v30],
            [v01, v11, v21, v31],
            [v02, v12, v22, v32],
            [v03, v13, v23, v33],
        ]
    }

    /// Convert this [`Matrix4x4`] into a flat array in row-major ordering
    pub fn into_row_array(self) -> [T; 16] {
        let [
            [v00, v01, v02, v03],
            [v10, v11, v12, v13],
            [v20, v21, v22, v23],
            [v30, v31, v32, v33],
        ] = self.v;

        [
            v00, v01, v02, v03, v10, v11, v12, v13, v20, v21, v22, v23, v30, v31, v32, v33,
        ]
    }

    /// Convert this [`Matrix4x4`] into a flat array in column-major ordering
    pub fn into_col_array(self) -> [T; 16] {
        let [
            [v00, v01, v02, v03],
            [v10, v11, v12, v13],
            [v20, v21, v22, v23],
            [v30, v31, v32, v33],
        ] = self.v;

        [
            v00, v10, v20, v30, v01, v11, v21, v31, v02, v12, v22, v32, v03, v13, v23, v33,
        ]
    }

    /// Convert this [`Matrix4x4`] into an array of [`Vector4`]s in row-major ordering
    pub fn into_row_vecs(self) -> [Vector4<T>; 4] {
        let [
            [v00, v01, v02, v03],
            [v10, v11, v12, v13],
            [v20, v21, v22, v23],
            [v30, v31, v32, v33],
        ] = self.v;

        [
            Vector4::new(v00, v01, v02, v03),
            Vector4::new(v10, v11, v12, v13),
            Vector4::new(v20, v21, v22, v23),
            Vector4::new(v30, v31, v32, v33),
        ]
    }

    /// Convert this [`Matrix4x4`] into an array of [`Vector4`]s in column-major ordering
    pub fn into_col_vecs(self) -> [Vector4<T>; 4] {
        let [
            [v00, v01, v02, v03],
            [v10, v11, v12, v13],
            [v20, v21, v22, v23],
            [v30, v31, v32, v33],
        ] = self.v;

        [
            Vector4::new(v00, v10, v20, v30),
            Vector4::new(v01, v11, v21, v31),
            Vector4::new(v02, v12, v22, v32),
            Vector4::new(v03, v13, v23, v33),
        ]
    }
}

impl<T> Into<[[T; 4]; 4]> for Matrix4x4<T> {
    fn into(self) -> [[T; 4]; 4] {
        self.into_row_arrays()
    }
}

impl<T> Into<[T; 16]> for Matrix4x4<T> {
    fn into(self) -> [T; 16] {
        self.into_row_array()
    }
}

impl<T> Into<[Vector4<T>; 4]> for Matrix4x4<T> {
    fn into(self) -> [Vector4<T>; 4] {
        self.into_row_vecs()
    }
}
