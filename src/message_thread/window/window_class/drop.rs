use crate::message_thread::window::WindowClass;
use std::ptr::null;
use win32::{GetModuleHandle, UnregisterClass, try_get_last_error};

impl Drop for WindowClass {
    fn drop(&mut self) {
        try_get_last_error!(unsafe { UnregisterClass(self.class as _, GetModuleHandle(null())) })
            .unwrap();
    }
}
