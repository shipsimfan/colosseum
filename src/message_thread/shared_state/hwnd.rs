use crate::message_thread::MessageThreadSharedState;
use std::sync::MutexGuard;
use win32::HWND;

impl MessageThreadSharedState {
    /// Get the window handle
    pub fn lock_hwnd<'a>(&'a self) -> MutexGuard<'a, Option<HWND>> {
        self.hwnd.lock().unwrap()
    }

    /// Set the window handle to be no longer valid
    pub fn set_valid_hwnd(&self, hwnd: HWND) {
        *self.hwnd.lock().unwrap() = Some(hwnd);
    }

    /// Set the window handle to be no longer valid
    pub fn invalidate_hwnd(&self) {
        *self.hwnd.lock().unwrap() = None;
    }
}
