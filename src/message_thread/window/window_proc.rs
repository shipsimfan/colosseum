use std::ptr::null_mut;

use crate::{
    debug,
    graphics::DisplayMode,
    input::KeyCode,
    math::{Vector2i, Vector2u},
    message_thread::{RawInputButtonEvent, WM_APP_SET_DISPLAY_MODE, WM_APP_SET_TITLE, Window},
    warning,
};
use win32::{
    DefWindowProc, GWLP_USERDATA, GetWindowLongPtr, HWND, LPARAM, LRESULT, UINT, WM_ACTIVATEAPP,
    WM_CLOSE, WM_ENTERSIZEMOVE, WM_EXITSIZEMOVE, WM_KEYDOWN, WM_KEYUP, WM_MOVE, WM_SIZE,
    WM_SYSKEYDOWN, WM_SYSKEYUP, WPARAM,
};

impl Window {
    /// Called to establish [`Window::window_proc`] as the main window procedure
    pub(in crate::message_thread::window) extern "system" fn init_window_proc(
        wnd: HWND,
        msg: UINT,
        w_param: WPARAM,
        l_param: LPARAM,
    ) -> LRESULT {
        let window_ptr = unsafe { GetWindowLongPtr(wnd, GWLP_USERDATA) };
        if window_ptr == 0 {
            unsafe { DefWindowProc(wnd, msg, w_param, l_param) }
        } else {
            unsafe { &mut *(window_ptr as *mut Window) }.window_proc(msg, w_param, l_param)
        }
    }

    /// Called when an event is consumed by the message pump
    pub(in crate::message_thread::window) fn window_proc(
        &mut self,
        msg: UINT,
        w_param: WPARAM,
        l_param: LPARAM,
    ) -> LRESULT {
        match msg {
            // The window is closing or the app is quiting
            WM_CLOSE => {
                self.running_state.kill();
                debug!(self.logger, "Close button pressed");
            }

            // The user has begun moving or resizing the window
            WM_ENTERSIZEMOVE => {
                self.in_move = true;
            }

            // The user has stopped moving or resizing the window
            WM_EXITSIZEMOVE => {
                self.in_move = false;
                self.shared_state.set_size(self.size);
                debug!(
                    self.logger,
                    "Window moved to {} and resized to {}x{}",
                    self.position,
                    self.size.x,
                    self.size.y
                );
            }

            // The window has changed size
            WM_SIZE => {
                let width = (l_param & 0xFFFF) as u32;
                let height = ((l_param >> 16) & 0xFFFF) as u32;
                self.size = Vector2u::new(width, height);

                if !self.in_move {
                    self.shared_state.set_size(self.size);
                    debug!(
                        self.logger,
                        "Window resized to {}x{}", self.size.x, self.size.y
                    );
                }
            }

            // The window has moved
            WM_MOVE => {
                let x = (l_param & 0xFFFF) as i16;
                let y = ((l_param >> 16) & 0xFFFF) as i16;
                self.position = Vector2i::new(x as _, y as _);
                self.shared_state.set_position(self.position);
            }

            // The window either gained or lost focus
            WM_ACTIVATEAPP => {
                self.is_focused = w_param != 0;
                self.shared_state.set_is_focused(self.is_focused);

                if self.is_focused {
                    debug!(self.logger, "Gained focus");
                } else {
                    debug!(self.logger, "Lost focus");
                }
            }

            // A key was pressed
            WM_KEYDOWN | WM_SYSKEYDOWN => {
                if !self.is_focused {
                    return 0;
                }

                if (l_param >> 30) & 1 == 1 {
                    return 0;
                }

                let scan_code =
                    ((l_param >> 16) & 0x1FF) as u16 | (((l_param >> 29) & 1) << 9) as u16;
                let key_code = match KeyCode::from_scan_code(scan_code) {
                    Some(key_code) => key_code,
                    None => {
                        warning!(
                            self.logger,
                            "Unknown scan code from keyboard: {}",
                            scan_code
                        );
                        return 0;
                    }
                };

                if self
                    .button_event_queue
                    .send(RawInputButtonEvent::new(null_mut(), key_code.into(), true))
                    .is_err()
                {
                    warning!(self.logger, "Dropped keyboard input");
                }
            }

            // A key was released
            WM_KEYUP | WM_SYSKEYUP => {
                if !self.is_focused {
                    return 0;
                }

                let scan_code =
                    ((l_param >> 16) & 0x1FF) as u16 | (((l_param >> 29) & 1) << 9) as u16;
                let key_code = match KeyCode::from_scan_code(scan_code) {
                    Some(key_code) => key_code,
                    None => {
                        warning!(
                            self.logger,
                            "Unknown scan code from keyboard: {}",
                            scan_code
                        );
                        return 0;
                    }
                };

                if self
                    .button_event_queue
                    .try_send(RawInputButtonEvent::new(null_mut(), key_code.into(), false))
                    .is_err()
                {
                    warning!(self.logger, "Dropped keyboard input");
                }
            }

            // A display mode change was requested
            WM_APP_SET_DISPLAY_MODE => {
                let display_mode = DisplayMode::from_w_param(w_param).unwrap();

                self.wnd_proc_result = self.handle.set_display_mode(display_mode);
            }

            // A title change was requested
            WM_APP_SET_TITLE => {
                let length = w_param & 0xFFFFFFFF;
                let capacity = w_param >> 32;
                let title =
                    unsafe { Vec::from_raw_parts(l_param as *mut u16, length as _, capacity as _) };

                self.wnd_proc_result = self.handle.set_title(&title);
            }

            // All other events
            _ => return unsafe { DefWindowProc(*self.handle, msg, w_param, l_param) },
        }

        0
    }
}
