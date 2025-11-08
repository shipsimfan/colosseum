use crate::MessageThread;
use win32::{PostThreadMessage, WM_QUIT, try_get_last_error};

impl Drop for MessageThread {
    fn drop(&mut self) {
        let hwnd_valid = self.shared_state.lock_hwnd_valid();
        if *hwnd_valid {
            try_get_last_error!(PostThreadMessage(self.thread_id, WM_QUIT, 0, 0)).unwrap();
        }
        drop(hwnd_valid);

        self.join_handle.take().unwrap().join().unwrap();
    }
}
