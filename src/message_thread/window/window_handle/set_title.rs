use crate::{Error, Result, message_thread::window::WindowHandle};
use win32::{SetWindowText, try_get_last_error};

impl WindowHandle {
    /// Set the window title
    #[allow(unused)]
    pub fn set_title(&mut self, title: &[u16]) -> Result<()> {
        assert!(title.len() > 0);
        assert_eq!(title[title.len() - 1], 0);

        try_get_last_error!(SetWindowText(self.handle, title.as_ptr()))
            .map_err(|os| Error::new_inner("unable to set window title", os))?;
        Ok(())
    }
}
