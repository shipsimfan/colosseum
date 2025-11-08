/// Returns the absolute value
pub trait Absolute {
    /// Returns the absolute value
    fn abs(self) -> Self;
}

// Signed integers
impl Absolute for i8 {
    fn abs(self) -> Self {
        i8::abs(self)
    }
}
impl Absolute for i16 {
    fn abs(self) -> Self {
        i16::abs(self)
    }
}
impl Absolute for i32 {
    fn abs(self) -> Self {
        i32::abs(self)
    }
}
impl Absolute for i64 {
    fn abs(self) -> Self {
        i64::abs(self)
    }
}
impl Absolute for i128 {
    fn abs(self) -> Self {
        i128::abs(self)
    }
}
impl Absolute for isize {
    fn abs(self) -> Self {
        isize::abs(self)
    }
}

// Floating-point
impl Absolute for f32 {
    fn abs(self) -> Self {
        f32::abs(self)
    }
}
impl Absolute for f64 {
    fn abs(self) -> Self {
        f64::abs(self)
    }
}
