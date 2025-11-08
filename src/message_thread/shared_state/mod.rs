use std::sync::{
    Mutex,
    atomic::{AtomicBool, AtomicU64},
};

use win32::HWND;

mod hwnd;
mod is_focused;
mod new;
mod position;
mod size;

/// The shared state of the message thread
pub(in crate::message_thread) struct MessageThreadSharedState {
    /// The handle to the window, in a thread safe container
    hwnd: Mutex<Option<HWND>>,

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

unsafe impl Send for MessageThreadSharedState {}
unsafe impl Sync for MessageThreadSharedState {}
