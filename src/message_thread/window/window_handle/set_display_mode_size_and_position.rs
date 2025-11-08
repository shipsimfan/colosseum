use crate::{
    Error, Result,
    graphics::DisplayMode,
    math::{Vector2i, Vector2u},
    message_thread::window::WindowHandle,
};
use std::ptr::null_mut;
use win32::{
    GWL_EXSTYLE, GWL_STYLE, GetLastError, SetLastError, SetWindowLong, SetWindowPos,
    try_get_last_error,
};

impl WindowHandle {
    /// Sets the display mode, position, and size of the window
    #[allow(unused)]
    pub fn set_display_mode_size_and_position(
        &self,
        display_mode: DisplayMode,
        size: Vector2u,
        position: Vector2i,
    ) -> Result<()> {
        let (size, position) = display_mode.client_to_window(size, position)?;
        let (style, ex_style) = display_mode.style();

        unsafe { SetLastError(0) };
        if unsafe { SetWindowLong(self.handle, GWL_STYLE, style as _) } == 0 {
            if unsafe { GetLastError() != 0 } {
                return Err(Error::new_inner(
                    "unable to set window style",
                    win32::Error::get_last_error(),
                ));
            }
        }

        if unsafe { SetWindowLong(self.handle, GWL_EXSTYLE, ex_style as _) } == 0 {
            if unsafe { GetLastError() != 0 } {
                return Err(Error::new_inner(
                    "unable to set extended window style",
                    win32::Error::get_last_error(),
                ));
            }
        }

        try_get_last_error!(SetWindowPos(
            self.handle,
            null_mut(),
            position.x,
            position.y,
            size.x as _,
            size.y as _,
            0
        ))
        .map_err(|os| Error::new_inner("unable to set window position and size", os))?;

        Ok(())
    }
}
