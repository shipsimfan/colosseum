/// Defines a zero value for a type
pub trait Zero {
    /// The zero value of this type
    const ZERO: Self;
}

// Unsigned integers
impl Zero for u8 {
    const ZERO: Self = 0;
}
impl Zero for u16 {
    const ZERO: Self = 0;
}
impl Zero for u32 {
    const ZERO: Self = 0;
}
impl Zero for u64 {
    const ZERO: Self = 0;
}
impl Zero for u128 {
    const ZERO: Self = 0;
}
impl Zero for usize {
    const ZERO: Self = 0;
}

// Signed integers
impl Zero for i8 {
    const ZERO: Self = 0;
}
impl Zero for i16 {
    const ZERO: Self = 0;
}
impl Zero for i32 {
    const ZERO: Self = 0;
}
impl Zero for i64 {
    const ZERO: Self = 0;
}
impl Zero for i128 {
    const ZERO: Self = 0;
}
impl Zero for isize {
    const ZERO: Self = 0;
}

// Floating-point
impl Zero for f32 {
    const ZERO: Self = 0.0;
}
impl Zero for f64 {
    const ZERO: Self = 0.0;
}

// Booleans
impl Zero for bool {
    const ZERO: Self = false;
}
