use crate::message_thread::Window;

impl Drop for Window {
    fn drop(&mut self) {
        self.shared_state.invalidate_hwnd();
    }
}
