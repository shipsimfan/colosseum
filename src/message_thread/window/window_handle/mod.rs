use win32::HWND;

mod create;
mod deref;
mod drop;
mod get_size_and_position;
mod set_display_mode_size_and_position;
mod set_title;

/// A Win32 window
pub(in crate::message_thread::window) struct WindowHandle {
    /// The handle to the window
    handle: HWND,
}
