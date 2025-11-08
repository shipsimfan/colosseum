/// An item which can be converted into an [`f32`]
pub trait IntoF32 {
    /// Convert this item into an [`f32`]
    fn into_f32(self) -> f32;
}

// Unsigned integers
impl IntoF32 for u8 {
    fn into_f32(self) -> f32 {
        self as f32
    }
}

impl IntoF32 for u16 {
    fn into_f32(self) -> f32 {
        self as f32
    }
}

impl IntoF32 for u32 {
    fn into_f32(self) -> f32 {
        self as f32
    }
}

impl IntoF32 for u64 {
    fn into_f32(self) -> f32 {
        self as f32
    }
}

impl IntoF32 for u128 {
    fn into_f32(self) -> f32 {
        self as f32
    }
}

impl IntoF32 for usize {
    fn into_f32(self) -> f32 {
        self as f32
    }
}

// Signed integers
impl IntoF32 for i8 {
    fn into_f32(self) -> f32 {
        self as f32
    }
}

impl IntoF32 for i16 {
    fn into_f32(self) -> f32 {
        self as f32
    }
}

impl IntoF32 for i32 {
    fn into_f32(self) -> f32 {
        self as f32
    }
}

impl IntoF32 for i64 {
    fn into_f32(self) -> f32 {
        self as f32
    }
}

impl IntoF32 for i128 {
    fn into_f32(self) -> f32 {
        self as f32
    }
}

impl IntoF32 for isize {
    fn into_f32(self) -> f32 {
        self as f32
    }
}

// Floating-point
impl IntoF32 for f32 {
    fn into_f32(self) -> f32 {
        self
    }
}

impl IntoF32 for f64 {
    fn into_f32(self) -> f32 {
        self as f32
    }
}

// Booleans
impl IntoF32 for bool {
    fn into_f32(self) -> f32 {
        if self {
            1.0
        } else {
            0.0
        }
    }
}
