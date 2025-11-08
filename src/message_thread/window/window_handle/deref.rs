use crate::message_thread::window::WindowHandle;
use std::ops::Deref;
use win32::HWND;

impl Deref for WindowHandle {
    type Target = HWND;

    fn deref(&self) -> &Self::Target {
        &self.handle
    }
}
