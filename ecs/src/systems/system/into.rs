use crate::System;

/// A trait for types that can be converted into a system
pub trait IntoSystem<Input> {
    /// The type of system this can be converted into
    type System: System;

    /// Convert this type into a system
    fn into_system(self) -> Self::System;
}
