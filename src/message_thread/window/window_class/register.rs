use crate::{
    Error, Result,
    message_thread::{Window, window::WindowClass},
};
use std::ptr::null;
use win32::{CS_HREDRAW, CS_OWNDC, CS_VREDRAW, GetModuleHandle, RegisterClassEx, WNDCLASSEX};

impl WindowClass {
    /// Register a new window class
    pub fn register(class_name: &[u16]) -> Result<Self> {
        assert!(class_name.last().is_some());
        assert_eq!(*class_name.last().unwrap(), 0);

        let wnd_class = WNDCLASSEX {
            style: CS_OWNDC | CS_HREDRAW | CS_VREDRAW,
            wnd_proc: Window::init_window_proc,
            instance: unsafe { GetModuleHandle(null()) },
            class_name: class_name.as_ptr(),
            ..Default::default()
        };

        let class = unsafe { RegisterClassEx(&wnd_class) };
        if class == 0 {
            return Err(Error::new_inner(
                "unable to register window class",
                win32::Error::get_last_error(),
            ));
        }

        Ok(WindowClass { class })
    }
}
