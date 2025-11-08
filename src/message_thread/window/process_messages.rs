use crate::{Error, Result, message_thread::Window};
use std::ptr::null_mut;
use win32::{DispatchMessage, GetMessage, MSG, PM_REMOVE, PeekMessage, TranslateMessage, WM_QUIT};

impl Window {
    /// Process all messages that have occurred since the last call
    ///
    /// If none have happened, this function will block until one occurs
    pub fn process_messages(&mut self) -> Result<()> {
        let mut msg = MSG::default();
        if unsafe { GetMessage(&mut msg, null_mut(), 0, 0) } == -1 {
            return Err(Error::new_inner(
                "unable to get messages",
                win32::Error::get_last_error(),
            ));
        }
        self.process_message(&msg)?;

        while self.is_running && unsafe { PeekMessage(&mut msg, null_mut(), 0, 0, PM_REMOVE) } != 0
        {
            self.process_message(&msg)?;
        }

        Ok(())
    }

    fn process_message(&mut self, msg: &MSG) -> Result<()> {
        if msg.message == WM_QUIT {
            self.is_running = false;
        }

        unsafe { TranslateMessage(msg) };
        unsafe { DispatchMessage(msg) };

        if self.wnd_proc_result.is_err() {
            let mut result = Ok(());
            std::mem::swap(&mut result, &mut self.wnd_proc_result);
            return result;
        }

        Ok(())
    }
}
