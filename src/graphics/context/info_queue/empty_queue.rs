use crate::{Error, Result, graphics::context::InfoQueue, log, logging::LogSeverity};
use std::{
    alloc::{Layout, alloc_zeroed, dealloc},
    borrow::Cow,
    ffi::CStr,
    ptr::null_mut,
};
use win32::{
    d3d11sdklayers::{D3D11_MESSAGE, D3D11_MESSAGE_SEVERITY},
    try_hresult,
};

impl InfoQueue {
    /// Empty all messages from the queue
    pub fn empty_queue(&mut self) -> Result<()> {
        let count = self.handle.get_num_stored_messages();
        for i in 0..count {
            // Get the message size
            let mut size = 0;
            try_hresult!(self.handle.get_message(i, null_mut(), &mut size)).map_err(|error| {
                Error::new_inner("unable to get info queue message size", error)
            })?;

            // Allocate space for the message
            let message_layout = Layout::from_size_align(
                std::mem::size_of::<D3D11_MESSAGE>() + size as usize,
                std::mem::align_of::<D3D11_MESSAGE>(),
            )
            .unwrap();
            let message_ptr = unsafe { alloc_zeroed(message_layout) };
            let message = unsafe { &mut *(message_ptr as *mut D3D11_MESSAGE) };

            // Get the message
            try_hresult!(self.handle.get_message(i, message, &mut size)).map_err(|error| {
                unsafe { dealloc(message_ptr, message_layout) };
                Error::new_inner("unable to get info queue message", error)
            })?;

            // Extract the information from the message
            let description = match unsafe { CStr::from_ptr(message.description) }.to_string_lossy()
            {
                Cow::Owned(owned) => owned,
                Cow::Borrowed(borrowed) => borrowed.to_string(),
            };
            let severity = match message.severity {
                D3D11_MESSAGE_SEVERITY::Corruption | D3D11_MESSAGE_SEVERITY::Error => {
                    LogSeverity::Error
                }
                D3D11_MESSAGE_SEVERITY::Warning => LogSeverity::Warning,
                D3D11_MESSAGE_SEVERITY::Info => LogSeverity::Info,
                _ => LogSeverity::Debug,
            };

            unsafe { dealloc(message_ptr, message_layout) };

            log!(severity, self.logger, "{}", description);
        }

        self.handle.clear_stored_messages();
        Ok(())
    }
}
