use win32::ATOM;

mod deref;
mod drop;
mod register;

/// The window class the game window belongs to
pub(in crate::message_thread::window) struct WindowClass {
    /// The handle to the window class
    class: ATOM,
}
