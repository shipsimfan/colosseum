use crate::{
    Error, Result,
    graphics::DisplayMode,
    math::{Vector2i, Vector2u},
    message_thread::{
        Window,
        window::{WindowClass, WindowHandle},
    },
};
use std::ptr::null_mut;
use win32::{CW_USEDEFAULT, CreateWindowEx, GWLP_USERDATA, GetModuleHandle, SetWindowLongPtr};

impl WindowHandle {
    /// Creates a new window and returns the handle to it
    pub fn create(
        title: &[u16],
        class: &WindowClass,
        x: Option<i32>,
        y: Option<i32>,
        width: Option<u32>,
        height: Option<u32>,
        display_mode: DisplayMode,
        window_ptr: *mut Window,
    ) -> Result<Self> {
        assert!(title.last().is_some());
        assert_eq!(*title.last().unwrap(), 0);

        let (style, ex_style) = display_mode.style();

        let (x, y, width, height) = match (width, height) {
            (Some(width), Some(height)) => {
                let (size, position) = display_mode.client_to_window(
                    Vector2u::new(width, height),
                    Vector2i::new(x.unwrap_or(0), y.unwrap_or(0)),
                )?;

                (
                    match x {
                        Some(_) => position.x,
                        None => CW_USEDEFAULT,
                    },
                    match y {
                        Some(_) => position.y,
                        None => CW_USEDEFAULT,
                    },
                    size.x as _,
                    size.y as _,
                )
            }
            (_, _) => (
                x.unwrap_or(CW_USEDEFAULT),
                y.unwrap_or(CW_USEDEFAULT),
                width.unwrap_or(CW_USEDEFAULT as _) as i32,
                height.unwrap_or(CW_USEDEFAULT as _) as i32,
            ),
        };

        let handle = unsafe {
            CreateWindowEx(
                ex_style,
                **class as _,
                title.as_ptr(),
                style,
                x,
                y,
                width,
                height,
                null_mut(),
                null_mut(),
                GetModuleHandle(null_mut()),
                null_mut(),
            )
        };

        if handle == null_mut() {
            return Err(Error::new_inner(
                "unable to create window",
                win32::Error::get_last_error(),
            ));
        }

        unsafe { SetWindowLongPtr(handle, GWLP_USERDATA, window_ptr as _) };

        Ok(WindowHandle { handle })
    }
}
