use crate::{
    Error, Result,
    math::{Vector2i, Vector2u},
    message_thread::window::WindowHandle,
};
use std::ptr::null_mut;
use win32::{
    GetClientRect, GetLastError, MapWindowPoints, RECT, SUCCEEDED, SetLastError, try_get_last_error,
};

impl WindowHandle {
    /// Gets the current position and size of the window
    pub fn get_size_and_position(&self) -> Result<(Vector2i, Vector2u)> {
        let mut rect = RECT::default();
        try_get_last_error!(GetClientRect(self.handle, &mut rect))
            .map_err(|os| Error::new_inner("unable to get window size", os))?;

        let size = Vector2u::new(rect.right as _, rect.bottom as _);

        unsafe { SetLastError(0) };
        unsafe { MapWindowPoints(self.handle, null_mut(), &mut rect as *mut RECT as _, 2) };
        if !SUCCEEDED!(unsafe { GetLastError() }) {
            return Err(Error::new_inner(
                "unable to adjust window coordinates",
                win32::Error::get_last_error(),
            ));
        }

        let position = Vector2i::new(rect.left, rect.top);

        Ok((position, size))
    }
}
