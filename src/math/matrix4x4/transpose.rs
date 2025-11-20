use crate::math::Matrix4x4;

impl<T> Matrix4x4<T> {
    /// Tranposes the values of the matrix
    pub fn transpose(&mut self) {
        for row in 0..4 {
            for col in row + 1..4 {
                let (upper, lower) = self.v.split_at_mut(col);
                std::mem::swap(&mut upper[row][col], &mut lower[0][row]);
            }
        }
    }

    /// Gets a transposed version of this matrix
    pub fn transposed(mut self) -> Matrix4x4<T> {
        self.transpose();
        self
    }
}

#[test]
fn simple_transpose() {
    const INPUT: Matrix4x4<u8> = Matrix4x4::new_row([
        [1, 2, 3, 4],
        [5, 6, 7, 8],
        [9, 10, 11, 12],
        [13, 14, 15, 16],
    ]);
    const TARGET: Matrix4x4<u8> = Matrix4x4::new_row([
        [1, 5, 9, 13],
        [2, 6, 10, 14],
        [3, 7, 11, 15],
        [4, 8, 12, 16],
    ]);

    assert_eq!(INPUT.transposed(), TARGET);
}
