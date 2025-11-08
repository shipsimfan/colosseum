use crate::message_thread::window::WindowHandle;
use win32::{DestroyWindow, GWLP_USERDATA, SetWindowLongPtr, try_hresult};

impl Drop for WindowHandle {
    fn drop(&mut self) {
        unsafe { SetWindowLongPtr(self.handle, GWLP_USERDATA, 0) };
        try_hresult!(DestroyWindow(self.handle)).unwrap();
    }
}
