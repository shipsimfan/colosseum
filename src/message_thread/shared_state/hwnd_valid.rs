use crate::message_thread::MessageThreadSharedState;
use std::sync::MutexGuard;

impl MessageThreadSharedState {
    /// Is the window handle still valid?
    pub fn lock_hwnd_valid<'a>(&'a self) -> MutexGuard<'a, bool> {
        self.hwnd_valid.lock().unwrap()
    }

    /// Set the window handle to be no longer valid
    pub fn invalidate_hwnd(&self) {
        *self.hwnd_valid.lock().unwrap() = false;
    }
}
