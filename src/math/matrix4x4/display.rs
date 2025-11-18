use crate::math::Matrix4x4;

impl<T: std::fmt::Display> std::fmt::Display for Matrix4x4<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        '['.fmt(f)?;
        for row in 0..4 {
            '['.fmt(f)?;
            for col in 0..4 {
                self.v[row][col].fmt(f)?;
                if col != 3 {
                    ", ".fmt(f)?;
                }
            }
            ']'.fmt(f)?;
            if row != 3 {
                ", ".fmt(f)?;
            }
        }
        ']'.fmt(f)
    }
}
