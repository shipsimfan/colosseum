use std::sync::{
    Mutex,
    atomic::{AtomicBool, AtomicU64},
};

mod hwnd_valid;
mod is_focused;
mod new;
mod position;
mod size;

/// The shared state of the message thread
pub(in crate::message_thread) struct MessageThreadSharedState {
    /// Is the HWND of the message thread still valid?
    hwnd_valid: Mutex<bool>,

    /// The position of the upper-left corner of the client area of the window
    ///
    /// The x position is stored as an [`i32`] in the lower 32-bits. The y position is stored as an
    /// [`i32`] in the upper 32-bits.
    position: AtomicU64,

    /// The size of the client area of the window
    ///
    /// The x size is stored as an [`u32`] in the lower 32-bits. The y size is stored as an [`u32`]
    /// in the upper 32-bits.
    size: AtomicU64,

    /// Is the window currently focused?
    is_focused: AtomicBool,
}
