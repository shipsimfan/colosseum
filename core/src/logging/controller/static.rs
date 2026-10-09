use crate::LogController;
use std::ptr::null;
use std::sync::Arc;

/// The global static instance of the log controller
static mut LOG_CONTROLLER: *const Arc<LogController> = null();

impl LogController {
    /// Get the global static instance of the log controller
    pub fn get() -> &'static Arc<LogController> {
        unsafe { LOG_CONTROLLER.as_ref().unwrap() }
    }

    /// Set the global static instance of the log controller
    pub(in crate::logging) fn set(controller: Arc<LogController>) {
        unsafe {
            if !LOG_CONTROLLER.is_null() {
                panic!("LogController has already been set");
            }

            LOG_CONTROLLER = Box::into_raw(Box::new(controller));
        }
    }
}
