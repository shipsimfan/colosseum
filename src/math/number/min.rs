/// Compares two values, returning the smallest
pub trait Min {
    /// Compares two values, returning the smallest
    fn min(self, other: Self) -> Self;
}

// Unsigned integers
impl Min for u8 {
    fn min(self, other: Self) -> Self {
        Ord::min(self, other)
    }
}
impl Min for u16 {
    fn min(self, other: Self) -> Self {
        Ord::min(self, other)
    }
}
impl Min for u32 {
    fn min(self, other: Self) -> Self {
        Ord::min(self, other)
    }
}
impl Min for u64 {
    fn min(self, other: Self) -> Self {
        Ord::min(self, other)
    }
}
impl Min for u128 {
    fn min(self, other: Self) -> Self {
        Ord::min(self, other)
    }
}
impl Min for usize {
    fn min(self, other: Self) -> Self {
        Ord::min(self, other)
    }
}

// Signed integers
impl Min for i8 {
    fn min(self, other: Self) -> Self {
        Ord::min(self, other)
    }
}
impl Min for i16 {
    fn min(self, other: Self) -> Self {
        Ord::min(self, other)
    }
}
impl Min for i32 {
    fn min(self, other: Self) -> Self {
        Ord::min(self, other)
    }
}
impl Min for i64 {
    fn min(self, other: Self) -> Self {
        Ord::min(self, other)
    }
}
impl Min for i128 {
    fn min(self, other: Self) -> Self {
        Ord::min(self, other)
    }
}
impl Min for isize {
    fn min(self, other: Self) -> Self {
        Ord::min(self, other)
    }
}

// Floating-point
impl Min for f32 {
    fn min(self, other: Self) -> Self {
        f32::min(self, other)
    }
}
impl Min for f64 {
    fn min(self, other: Self) -> Self {
        f64::min(self, other)
    }
}

// Booleans
impl Min for bool {
    fn min(self, other: Self) -> Self {
        Ord::min(self, other)
    }
}
