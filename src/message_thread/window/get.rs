use crate::message_thread::Window;
use win32::HWND;

impl Window {
    /// Is the window still running?
    pub fn is_running(&self) -> bool {
        self.is_running
    }

    /// Get the underlying handle to the window
    pub(crate) fn handle(&self) -> HWND {
        *self.handle
    }
}
