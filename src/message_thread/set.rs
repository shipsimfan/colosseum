use crate::{
    Error, Result,
    graphics::DisplayMode,
    math::{Vector2i, Vector2u},
    message_thread::{MessageThread, WM_APP_SET_DISPLAY_MODE, WM_APP_SET_TITLE},
};
use std::ptr::null_mut;
use win32::{PostMessage, SWP_ASYNCWINDOWPOS, SetWindowPos, try_get_last_error};

impl MessageThread {
    /// Set the size and position of the window
    pub fn set_size_and_position(&self, size: Vector2u, position: Vector2i) -> Result<()> {
        let hwnd = self.shared_state.lock_hwnd();
        if let Some(hwnd) = *hwnd {
            try_get_last_error!(SetWindowPos(
                hwnd,
                null_mut(),
                position.x,
                position.y,
                size.x as _,
                size.y as _,
                SWP_ASYNCWINDOWPOS
            ))
            .map_err(|error| Error::new_inner("unable to set window size and position", error))?;
        }
        Ok(())
    }

    /// Set the display mode of the window
    pub fn set_display_mode(&self, display_mode: DisplayMode) -> Result<()> {
        let hwnd = self.shared_state.lock_hwnd();
        if let Some(hwnd) = *hwnd {
            try_get_last_error!(PostMessage(
                hwnd,
                WM_APP_SET_DISPLAY_MODE,
                display_mode as _,
                0
            ))
            .map_err(|error| Error::new_inner("unable to set window title", error))?;
        }
        Ok(())
    }

    /// Set the title of the window
    pub fn set_window_title(&self, title: Vec<u16>) -> Result<()> {
        let hwnd = self.shared_state.lock_hwnd();
        if let Some(hwnd) = *hwnd {
            let (ptr, length, capacity) = title.into_raw_parts();
            assert!(length <= u32::MAX as _);
            assert!(capacity <= u32::MAX as _);

            try_get_last_error!(PostMessage(
                hwnd,
                WM_APP_SET_TITLE,
                length as u64 | ((capacity as u64) << 32),
                ptr as _,
            ))
            .map_err(|error| Error::new_inner("unable to set window title", error))?;
        }
        Ok(())
    }
}
