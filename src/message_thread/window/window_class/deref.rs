use crate::message_thread::window::WindowClass;
use std::ops::Deref;
use win32::ATOM;

impl Deref for WindowClass {
    type Target = ATOM;

    fn deref(&self) -> &Self::Target {
        &self.class
    }
}
