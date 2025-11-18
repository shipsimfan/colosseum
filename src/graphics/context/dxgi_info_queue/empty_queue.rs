use crate::{Error, Result, graphics::context::DXGIInfoQueue, log, logging::LogSeverity};
use std::{
    alloc::{Layout, alloc_zeroed, dealloc},
    borrow::Cow,
    ffi::CStr,
    ptr::null_mut,
};
use win32::{
    dxgidebug::{DXGI_DEBUG_ALL, DXGI_INFO_QUEUE_MESSAGE, DXGI_INFO_QUEUE_MESSAGE_SEVERITY},
    try_hresult,
};

impl DXGIInfoQueue {
    /// Empty all messages from the queue
    pub fn empty_queue(&mut self) -> Result<()> {
        let count = self.handle.get_num_stored_messages(DXGI_DEBUG_ALL);
        for i in 0..count {
            // Get the message size
            let mut size = 0;
            try_hresult!(
                self.handle
                    .get_message(DXGI_DEBUG_ALL, i, null_mut(), &mut size)
            )
            .map_err(|error| Error::new_inner("unable to get info queue message size", error))?;

            // Allocate space for the message
            let message_layout = Layout::from_size_align(
                std::mem::size_of::<DXGI_INFO_QUEUE_MESSAGE>() + size as usize,
                std::mem::align_of::<DXGI_INFO_QUEUE_MESSAGE>(),
            )
            .unwrap();
            let message_ptr = unsafe { alloc_zeroed(message_layout) };
            let message = unsafe { &mut *(message_ptr as *mut DXGI_INFO_QUEUE_MESSAGE) };

            // Get the message
            try_hresult!(
                self.handle
                    .get_message(DXGI_DEBUG_ALL, i, message, &mut size)
            )
            .map_err(|error| {
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
                DXGI_INFO_QUEUE_MESSAGE_SEVERITY::Corruption
                | DXGI_INFO_QUEUE_MESSAGE_SEVERITY::Error => LogSeverity::Error,
                DXGI_INFO_QUEUE_MESSAGE_SEVERITY::Warning => LogSeverity::Warning,
                DXGI_INFO_QUEUE_MESSAGE_SEVERITY::Info => LogSeverity::Info,
                _ => LogSeverity::Debug,
            };

            unsafe { dealloc(message_ptr, message_layout) };

            log!(severity, self.logger, "{}", description);
        }

        self.handle.clear_stored_messages(DXGI_DEBUG_ALL);
        Ok(())
    }
}
